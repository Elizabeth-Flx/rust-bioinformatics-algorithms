fn main() {

    let text = "AABAACAADAABAABA";
    let keyword = "AABA";

    compute_borders("ababaabb");

    let mut alignment_pos = naive_substring_finder(text, keyword);

    print_alignment(text, keyword, alignment_pos);

    alignment_pos = knuth_morris_pratt(text, keyword);

    print_alignment(text, keyword, alignment_pos);
}

fn print_alignment(text: &str, keyword: &str, index: i32) {
    println!("{}", text);
    for _ in 0..index {
        print!(" ");
    }
    println!("{}", keyword);
}




fn naive_substring_finder(text: &str, keyword: &str) -> i32 {

    let text_chars: Vec<char> = text.chars().collect();
    let text_len: usize = text.len();

    let keyword_chars: Vec<char>  = keyword.chars().collect();
    let keyword_len: usize = keyword.len();
    
    for i in 0..=(text_len-keyword_len) {
        for j in 0..keyword_len {
            if text_chars[i+j] != keyword_chars[j] {
                break;
            }
            if j == keyword_len-1 {
                return i as i32;
            }
        }
    }
    return -1;
}


fn compute_borders(keyword: &str) -> Vec<i32> {
    let keyword_chars: Vec<char> = keyword.chars().collect();
    let keyword_len: usize = keyword.len();

    let mut borders: Vec<i32> = Vec::new();
    
    borders.push(-1);
    borders.push(0);

    let mut i: i32 = borders[1];

    for j in 2..=keyword_len {
        while i >= 0 && keyword_chars[i as usize] != keyword_chars[j-1 as usize] {
            i = borders[i as usize]
        }
        i += 1;
        borders.push(i)
    }
    
    println!("{:?}", borders);
    return borders;
}

fn knuth_morris_pratt(text: &str, keyword: &str) -> i32 {

    let text_chars: Vec<char> = text.chars().collect();
    let text_len: usize = text.len();

    let keyword_chars: Vec<char> = keyword.chars().collect();
    let keyword_len: usize = keyword.len();

    let borders: Vec<i32> = compute_borders(keyword); 

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i <= text_len-keyword_len {
        while text_chars[i+j] == keyword_chars[j] {
            j += 1;
            if j == keyword_len {
                return i as i32;
            }
        }
        i += j - borders[j] as usize;
        j = if borders[j] >= 0 { borders[j] as usize } else { 0 };
    }
    return -1;
}


