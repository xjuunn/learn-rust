mod exercises;

fn main() {
    println!("=== 歌词行号生成器 ===");
    let lyric = "夜空中最亮的星\n能否听清\n那仰望的人\n心底的孤独和叹息";
    let lines = exercises::current::exercise_fn(lyric);
    for line in lines {
        println!("  {}", line);
    }
}