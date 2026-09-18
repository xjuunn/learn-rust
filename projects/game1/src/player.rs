// =====================================================================================
// 玩家模块（player.rs）—— 负责三件事：
//   1) 生成玩家实体（挂上精灵图、初始状态、动画定时器）
//   2) 读取键盘输入，移动玩家并更新朝向
//   3) 根据移动状态，逐帧推进精灵图动画（行走 / 静止）
//
// 本文件几乎涵盖了 Bevy 2D 游戏的全部核心概念：插件、系统、组件、查询、资源、
// 资产加载、精灵图（Spritesheet）、纹理图集（TextureAtlas）、定时器、输入。
// 文件末尾有《附录：用到的 Bevy 模块 / API 详解》，列出每个类型的完整常用方法。
// =====================================================================================

// 导入 Bevy 的预置项（prelude），以及两个显式单独导入的项。
//
// 1) prelude::*：
//    Bevy 把最常用的类型/trait/宏集中 re-export 到 prelude 里，一句就能拿到绝大部分日常 API。
//    包括：App、Commands、Query、Res、ResMut、Component、Plugin、Startup、Update、
//          Sprite、Transform、Vec2、Vec3、Color、Time、Timer、KeyCode、ButtonInput 等。
use bevy::prelude::*;

// -------------------------------------------------------------------------------------
// 插件（Plugin）：玩家功能的封装
// -------------------------------------------------------------------------------------

// 定义一个"玩家插件"结构体。空结构体（field-less struct）不携带任何数据，
// 它的唯一作用就是作为一个"功能模块"的入口标记。
// 为什么用插件：Bevy 用插件组织代码——把"一组系统 + 资源 + 初始化逻辑"打包成一个可复用单元。
// 之后在 main.rs 里写 add_plugins(PlayerPlugin)，玩家的全部系统就一次性装进应用。
pub struct PlayerPlugin;

// 为 PlayerPlugin 实现 Bevy 的 Plugin trait。
// trait 是 Rust 的"接口"概念：实现了它，Bevy 才知道"这个类型是插件、该如何加载我"。
// Plugin trait 必须实现唯一的必需方法 build。
impl Plugin for PlayerPlugin {
    // build 在插件被 add_plugins 时调用一次，用来往 App 上注册系统、资源等。
    // &self 是插件自身（这里不需要读它的数据）；app: &mut App 是应用构建器的可变引用。
    fn build(&self, app: &mut App) {
        // 把 spawn_player 注册到 Startup 阶段：应用启动时执行且仅执行一次。
        // 适合"初始化"类工作（生成相机、玩家、加载资产）。
        app.add_systems(Startup, spawn_player)
            // 链式调用继续注册到 Update 阶段：主循环每一帧都执行，适合游戏逻辑。
            // 参数写成元组 (move_player, animate_player)，一次注册多个系统。
            // 注意：这个调用返回 &mut App，因此可以像这样连点（builder 风格）。
            .add_systems(Update, (move_player, animate_player));
    }
}

// -------------------------------------------------------------------------------------
// 常量：所有可调参数集中定义，方便统一调参
// -------------------------------------------------------------------------------------

// 精灵图（Spritesheet）里单个瓦片（tile）= 单帧画面的边长，单位像素。
// 精灵图是一张"把角色所有动画帧拼在一起"的大图，切割后每块 64x64。
const TILE_SIZE: u32 = 64;

// 每行"行走动画"包含的帧数。本图集每行前 9 格是行走帧，之后是其它帧。
// 之所以是 9：后面的取模运算 % WALK_FRAMES 与行起始偏移都以它为一个"行周期"。
const WALK_FRAMES: usize = 9;

// 玩家移动速度，单位是"像素 / 秒"（逻辑像素）。
// 配合 delta_secs() 使用，使速度与帧率无关。
const MOVE_SPEED: f32 = 140.0;

// 动画切换间隔，单位秒。0.1 秒/帧 ≈ 10 FPS 的播放节奏，模拟复古像素游戏步频。
const ANIM_DT: f32 = 0.1;

// -------------------------------------------------------------------------------------
// 组件（Component）：挂在实体上的数据
// -------------------------------------------------------------------------------------

// Player 是"标记组件（marker component）"：零字段、不存任何数据。
// 作用只是给实体贴一个身份标签，让系统能用 With<Player> 精确筛出玩家实体。
// #[derive(Component)] 宏为它生成 Bevy 所需的 trait 实现。
#[derive(Component)]
struct Player;

// 朝向枚举，同样是组件。
// 派生（derive）的每个 trait 各有用处：
//   - Component    : 让它能作为组件挂在实体上
//   - Debug        : 允许 println!("{:?}", facing) 调试输出
//   - Clone, Copy  : 这个枚举极小（1 字节），复制成本近乎为零；Copy 后赋值是"复制"而非"移动"，
//                    避免在移动系统里因为所有权移动而无法再次使用该值
//   - PartialEq, Eq: 允许用 == / != 比较（后面用 current_row != target_row 之类的逻辑思路相同）
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,    // 面朝上（精灵图对应行）
    Left,  // 面朝左
    Down,  // 面朝下
    Right, // 面朝右
}

// Newtype（新类型）模式：用元组结构体包裹一个标准库/引擎类型，赋予它独立身份。
// 这里包裹 Timer，意义有二：
//   1) 让"动画定时器"成为一个独立组件类型。系统可以写 Query<..., With<AnimationTimer>>，
//      不会和实体上其它 Timer 混淆；同时同一实体允许挂多个不同用途的计时器。
//   2) #[derive(Deref, DerefMut)] 会生成解引用实现：把 AnimationTimer 当成 Timer 用。
//      于是 timer.tick(...)、timer.reset() 这些 Timer 方法可以直接调用，
//      编译器自动插入"先解引用到内部 Timer"的步骤，零运行时开销。
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// 动画状态组件：记录播放行走动画所需的所有外部状态。
// 把这些字段放进组件而不是散落的全局变量，是 ECS 的核心思想——数据跟着实体走。
#[derive(Component)]
struct AnimationState {
    facing: Facing,   // 当前面朝方向：决定播放精灵图的哪一行
    moving: bool,     // 本帧玩家是否正在移动：决定播行走帧还是停住
    was_moving: bool, // 上一帧是否在移动：通过对比才能检测"刚起步 / 刚停下"两个瞬间
}

// -------------------------------------------------------------------------------------
// 系统一：生成玩家（Startup 阶段，只跑一次）
// -------------------------------------------------------------------------------------

// 系统（System）本质是一个普通函数，参数由 Bevy 运行时自动注入（依赖注入）。
// 参数：
//   mut commands: Commands                  —— 实体/组件操作队列（延迟到本帧稍后统一执行）
//   asset_server: Res<AssetServer>          —— 资产加载服务（只读资源）
//   mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
//                                           —— 纹理图集布局的"内存仓库"（可写资源），
//                                              add() 会登记布局并返回 Handle 句柄
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 加载精灵图文件。关键理解：load 是**异步**的。
    // 它不会阻塞等待文件读完，而是立刻返回一个 Handle<Image>（句柄 / 取件凭证）。
    // 真正用到图片时，Bevy 会在后台线程完成读取并填充到 Assets<Image> 仓库里。
    // 路径相对于"资产根目录"；本项目在 main.rs 里用 AssetPlugin{ file_path: "src/assets" }
    // 把它改成了 src/assets，所以这里写的是相对该目录的文件名。
    let texture = asset_server.load("character-spritesheet.png");

    // 把整张精灵图按"规则网格"切分成一块块等大的瓦片（tile），生成切分方案 layout。
    // TextureAtlasLayout::from_grid 参数依次为：
    //   tile_size : UVec2  —— 每个瓦片的像素尺寸（这里 64x64；UVec2::splat(64) 表示两个分量都是 64）
    //   columns   : u32    —— 网格列数（每行几帧，这里 WALK_FRAMES = 9）
    //   rows      : u32    —— 网格行数（这里 12 行）
    //   padding   : Option<UVec2> —— 瓦片之间的内边距（None = 紧贴无间隙）
    //   offset    : Option<UVec2> —— 网格相对图片左上角的整体偏移（None = 从 0,0 开始）
    // 切分是纯数据计算（算出一堆矩形 URect），结果加入 Assets 仓库后得到 layout 句柄。
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE), // 每个瓦片 64x64 像素
        WALK_FRAMES as u32,      // 每行 9 列（WALK_FRAMES 是 usize，这里显式 as 成 u32）
        12,                      // 共 12 行
        None,                    // 瓦片间无内边距
        None,                    // 无整体偏移
    ));

    // 出生时的初始朝向：朝下（面向屏幕下方）。
    let facing = Facing::Down;

    // 由"朝向 + 行内第几帧"换算出精灵图的全局帧序号，这里取朝下行的第 0 帧。
    let start_index = atlas_index_for(facing, 0);

    // spawn 生成实体。它的参数是一个"组件元组（Bundle）"，
    // 元组里每个元素都会作为组件挂到这个新实体上。顺序无关。
    commands.spawn((
        // ============ 1. 精灵渲染组件 ============
        // Sprite::from_atlas_image(image, atlas) 表示"显示这张精灵图中的某一帧"：
        //   image：精灵图大图的句柄
        //   TextureAtlas { layout, index }：
        //       layout —— 用哪套切分方案（切好的矩形表）
        //       index  —— 当前显示第几块瓦片（全局线性编号，从 0 开始）
        // 渲染时引擎按 index 去 layout 里查出对应矩形，从大图中"抠"出这一小块来画。
        // 所谓逐帧动画，本质上就是"按时间改变 index"。
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        // ============ 2. 空间变换组件 ============
        // Transform 描述实体在 2D/3D 世界中的位置 / 旋转 / 缩放。
        // translation（平移）是位置，Vec3::ZERO 表示放在世界原点 (0, 0, 0)。
        // 2D 游戏中 z 用于渲染层级（z 越大越靠前看）。
        Transform::from_translation(Vec3::ZERO),
        // ============ 3. 身份标记组件 ============
        // 贴上 Player 标签，其它系统才能用 With<Player> 找到"这个就是玩家"。
        Player,
        // ============ 4. 动画状态组件 ============
        // 初始化动画状态：朝下、本帧未移动、上帧也未移动（出生时静止站立）。
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        // ============ 5. 动画定时器组件 ============
        // Timer::from_seconds(时长秒, 模式)：
        //   一个每隔 ANIM_DT(0.1s) 就"滴答"触发一次的循环定时器（TimerMode::Repeating）。
        // 每帧由 animate_player 手动 tick 推进它；每当走满 0.1 秒就切一帧。
        // 这个"节拍器"把动画节奏从游戏帧率中解耦：无论 30 帧还是 144 帧，动画时长恒定。
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

// -------------------------------------------------------------------------------------
// 系统二：移动玩家（Update 阶段，每帧）
// -------------------------------------------------------------------------------------

// 参数：
//   input: Res<ButtonInput<KeyCode>> —— 键盘输入状态资源
//   time: Res<Time>                  —— 时间资源（提供上一帧耗时）
//   mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>
//       查询所有带 Player 标记的实体，取出它们的 Transform 与 AnimationState（&mut = 可修改）
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    // let-else 模式匹配：尝试取得"唯一匹配实体"的三个组件。
    // single_mut() 约定查询结果恰好一个；如果结果是 0 个或多个，会返回 Err。
    // 用 let Ok(...) = ... else { return } 在失败时直接退出系统，
    // 比 unwrap() 更安全（不会 panic 崩溃）；"玩家还没生成"的瞬态理应被安静跳过。
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    // 方向向量：表示"这一帧想往哪个方向走"。
    // Vec2::ZERO 是 (0, 0) 常量，代表"没有按任何方向键"。
    let mut direction = Vec2::ZERO;

    // ---- 输入累积：把 A/S/D/W 映射为上下左右 ----
    // 注意这里用的是 4 个独立 if（而非 else if）：
    //   独立 if 允许同时按住两个键，方向向量会叠加（如 W+D 得到 (1, 1) 的斜向）；
    //   若用 else if，则只有一个分支会执行，最后一个命中的键会"吃掉"前面的键。
    //
    // input.pressed(key)：key 当前是否被按住（按住期间每帧都为 true）。
    // 移动类操作用 pressed；跳跃/射击这类"只触发一次"的操作用 just_pressed 更合适。
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0; // A：向左 → x 减 1（屏幕坐标 x 轴向右为正）
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0; // D：向右 → x 加 1
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0; // W：向上 → y 加 1（Bevy 2D 中 y 轴向上为正）
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0; // S：向下 → y 减 1
    }

    // 只有方向向量非零（真有键被按下）才执行移动逻辑，否则原地待命。
    if direction != Vec2::ZERO {
        // ============ 精妙点 1：归一化（normalize）============
        // normalize() 把向量变成"方向不变、长度恰好为 1"的单位向量。
        // 为什么必须做：如果同时按 W+D，direction 是 (1,1)，它的真实长度是 √2 ≈ 1.414。
        // 若直接乘速度，斜向移动会比直向移动快 41%，手感很怪。
        // 归一化后统一按"单位长度 x 速度"位移，四面八方速度一致。
        //
        // ============ 精妙点 2：帧率无关（delta time）============
        // time.delta_secs() 返回"上一帧经过了多少秒"。
        // 位移 = 速度(像素/秒) x 时间(秒) 是物理公式。
        // 把它乘进来后，1 秒内无论渲染多少帧，移动距离都恒为 MOVE_SPEED 像素。
        // 若不做这步（每帧固定位移），帧率高的机器角色会跑得飞快。
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        transform.translation.x += delta.x; // 把本帧 x 位移累加到当前位置
        transform.translation.y += delta.y; // 把本帧 y 位移累加到当前位置
        anim.moving = true;                 // 标记"正在移动"，供动画系统播行走帧

        // ============ 精妙点 3：按位移分量大小决定朝向 ============
        // 斜向移动时（如右上），x 与 y 同时非零，只能选一个"主导方向"作为面朝方向。
        // 用 |x| > |y| 判断：水平分量占优就面朝左右，否则面朝上下。
        // 这样避免了"在两个斜向朝向之间每帧抖动"的问题。
        if direction.x.abs() > direction.y.abs() {
            // 水平占优：x 为正朝右，x 为负朝左（用三元式 if 表达式赋值）
            anim.facing = if direction.x > 0.0 {Facing::Right} else {Facing::Left}
        } else {
            // 竖直占优：y 为正朝上，y 为负朝下
            anim.facing = if direction.y > 0.0 {Facing::Up} else {Facing::Down}
        }
    } else {
        // 没有任何方向键：标记为"未移动"。动画系统会让角色定格在当前帧（静立）。
        anim.moving = false;
    }
}

// -------------------------------------------------------------------------------------
// 系统三：播放行走动画（Update 阶段，每帧）
// -------------------------------------------------------------------------------------

// 参数：
//   time: Res<Time> —— 提供每帧时间差，用于推进定时器
//   query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>
//        取出玩家的动画状态、定时器、精灵（要修改 sprite.texture_atlas 里的 index）
fn animate_player(time: Res<Time>, mut query: Query<(&mut AnimationState, &mut AnimationTimer,&mut Sprite), With<Player>>) {
    // 同样的单例守卫：找不到玩家就跳过（不 panic）。
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    // 从精灵中取出纹理图集（TextureAtlas）的可变引用。
    // sprite.texture_atlas 是 Option<TextureAtlas>；as_mut() 得到 Option<&mut TextureAtlas>。
    // 这里用 match 做"有则用、无则退出"的处理：
    //   理论上玩家一定有图集，但编译器要求处理 None 分支；写成 return 也最安全。
    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    // 目标行 = 当前朝向对应的精灵图行号（0 基）。
    // 例如 Facing::Down 对应第 10 行，那么该行所有帧的全局编号是 [90, 98]。
    let target_row = row_zero_based(anim.facing);

    // ============ 精妙点 4：把全局线性索引拆回"行 + 列" ============
    // atlas.index 是整张大图中瓦片的一维编号（第 0、1、2 … 块，从左到右、从上到下）。
    // 因为图集按行排布，满足：index = 行号 x 每行帧数 + 列号，于是：
    //   index % WALK_FRAMES = 当前在第几列（行内第几帧）
    //   index / WALK_FRAMES = 当前在第几行（属于哪个朝向）
    let mut current_col = atlas.index % WALK_FRAMES;
    // 之所以用除法：WALK_FRAMES 是每行固定帧数，整除即行号（整数除法自动向下取整）
    let mut current_row = atlas.index / WALK_FRAMES;

    // 如果当前行不是目标行（说明玩家这一帧改变了朝向），必须"跳行"。
    if current_row != target_row {
        // 跳到目标行的第一帧：行号 x 每行帧数。
        atlas.index = row_start_index(anim.facing);
        current_col = 0;           // 同步本地列号（避免下面继续用旧值）
        current_row = target_row;  // 同步本地行号
        timer.reset();             // 重置节拍，让新方向的动画从第 0 帧重新开始，不会半路接续
    }

    // 检测两个"状态突变瞬间"：
    //   just_started：本帧在移动，但上一帧不在移动 —— 刚起步
    //   just_stopped：本帧不在移动，但上一帧在移动 —— 刚停下
    // 用 bool 逻辑与（&&）组合，两者都由 existing 字段推导，稳定可靠。
    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;

    if anim.moving {
        if just_started {
            // ============ 精妙点 5：起步立即推进一帧 ============
            // 玩家刚按下的那一瞬间，不等定时器走满 0.1 秒，立刻切到下一帧。
            // 视觉上"手一按，脚立刻抬起"，响应感强。
            // 若傻等定时器，最快也有 0.1 秒的僵硬延迟。
            let row_start = row_start_index(anim.facing);
            // 列号 +1 后对 WALK_FRAMES 取模：行尾（第 9 帧）会回到第 0 帧，形成循环播放
            let next_col = (current_col + 1) % WALK_FRAMES;
            // 全局编号 = 行起始编号 + 行内列号
            atlas.index = row_start + next_col;
            timer.reset(); // 重置定时器，让下一次切帧重新计满 0.1 秒
        } else {
            // ============ 平常行走的节拍逻辑 ============
            // 推进定时器：tick 需要传入"这一帧过了多久"来累加已用时间。
            timer.tick(time.delta());
            // just_finished() 只在该次 tick 恰好走满周期的那一帧返回 true，
            // 是"帧边界检测"的惯用写法，保证大约每 0.1 秒准确切一帧。
            if timer.just_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col; // 前进一帧
            }
        }
    } else if just_stopped {
        // 刚停下的那一帧：重置定时器。
        // 好处是下次再起步时从"崭新计时"开始，而不是接着上次剩余的进度，节拍更干净。
        timer.reset();
    }

    // 关键收尾：把本帧的 moving 保存为 was_moving，供下一帧判断"刚起步 / 刚停下"。
    // 忘记这行的话，just_started 会永远为 false（因为 was_moving 永远是 false 或旧值）。
    anim.was_moving = anim.moving;
}

// -------------------------------------------------------------------------------------
// 辅助函数：精灵图索引换算（纯计算，无副作用）
// -------------------------------------------------------------------------------------

// 给定朝向，返回"该朝向所在行的全局起始帧序号"。
// 公式：行号（0 基） x 每行帧数。
// 例：Down 是第 10 行 -> 10 x 9 = 90，即朝下动画从第 90 块瓦片开始。
fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

// 把"朝向 + 行内帧号"换算成精灵图全局帧序号，供出生 / 复位时使用。
// frame_in_row.min(WALK_FRAMES - 1) 是防御性截断：
//   即使调用方不小心传入超出 0..=8 的行内帧号（比如 100），
//   也会被钳制到最后一帧（8），绝不会越界到下一行（变成别的朝向）。
// 输入：facing 朝向；frame_in_row 行内第几帧（0 基）
// 输出：该帧在整张精灵图中的全局线性编号
fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES -1)
}

// 把 Facing 枚举映射为它在精灵图中的行号（0 基，即"第 0 行"代表图片最上面一行）。
// 这是一张纯粹的业务映射表，完全取决于美术素材的排布约定：
//   第 8 行 -> Up、第 9 行 -> Left、第 10 行 -> Down、第 11 行 -> Right
// 注意 match 必须穷尽所有枚举变体，这也是 Rust 借用枚举保证安全的一点体现。
fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing:: Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}