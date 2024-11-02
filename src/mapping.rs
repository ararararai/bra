// src/mapping.rs
use std::collections::HashMap;

pub fn create_mapping() -> HashMap<char, [bool; 6]> {
    let mut mapping = HashMap::new();

    // あ行
    mapping.insert('あ', [true, false, false, false, false, false]);
    mapping.insert('い', [true, false, false, true, false, false]);
    mapping.insert('う', [true, true, false, false, false, false]);
    mapping.insert('え', [true, true, false, true, false, false]);
    mapping.insert('お', [true, false, false, true, true, false]);

    // か行
    mapping.insert('か', [true, false, true, false, false, false]);
    mapping.insert('き', [true, false, true, true, false, false]);
    mapping.insert('く', [true, true, true, false, false, false]);
    mapping.insert('け', [true, true, true, true, false, false]);
    mapping.insert('こ', [true, false, true, true, true, false]);

    // さ行
    mapping.insert('さ', [true, false, false, false, true, false]);
    mapping.insert('し', [true, false, false, true, true, false]);
    mapping.insert('す', [true, true, false, false, true, false]);
    mapping.insert('せ', [true, true, false, true, true, false]);
    mapping.insert('そ', [true, false, false, true, true, true]);

    // た行
    mapping.insert('た', [true, false, true, false, true, false]);
    mapping.insert('ち', [true, false, true, true, true, false]);
    mapping.insert('つ', [true, true, true, false, true, false]);
    mapping.insert('て', [true, true, true, true, true, false]);
    mapping.insert('と', [true, false, true, true, true, true]);

    // な行
    mapping.insert('な', [true, false, false, false, false, true]);
    mapping.insert('に', [true, false, false, true, false, true]);
    mapping.insert('ぬ', [true, true, false, false, false, true]);
    mapping.insert('ね', [true, true, false, true, false, true]);
    mapping.insert('の', [true, false, false, true, true, true]);

    // は行
    mapping.insert('は', [false, true, true, false, false, false]);
    mapping.insert('ひ', [false, true, true, true, false, false]);
    mapping.insert('ふ', [false, true, true, false, true, false]);
    mapping.insert('へ', [false, true, true, true, true, false]);
    mapping.insert('ほ', [false, true, true, true, true, true]);

    // ま行
    mapping.insert('ま', [false, true, false, false, true, false]);
    mapping.insert('み', [false, true, false, true, true, false]);
    mapping.insert('む', [false, true, true, false, true, false]);
    mapping.insert('め', [false, true, true, true, true, false]);
    mapping.insert('も', [false, true, false, true, true, true]);

    // や行
    mapping.insert('や', [false, true, false, false, false, true]);
    mapping.insert('ゆ', [false, true, true, false, false, true]);
    mapping.insert('よ', [false, true, false, true, false, true]);

    // ら行
    mapping.insert('ら', [false, true, false, false, true, true]);
    mapping.insert('り', [false, true, false, true, true, true]);
    mapping.insert('る', [false, true, true, false, true, true]);
    mapping.insert('れ', [false, true, true, true, true, true]);
    mapping.insert('ろ', [false, true, false, true, true, true]);

    // わ行
    mapping.insert('わ', [false, false, false, false, true, true]);
    mapping.insert('を', [false, false, false, true, true, true]);
    mapping.insert('ん', [false, false, true, false, true, true]);

    // 句読点
    mapping.insert('。', [false, false, true, false, false, true]);
    mapping.insert('、', [false, false, true, false, false, false]);

    // 空白（表示用の特殊パターン）
    mapping.insert(' ', [false, false, false, false, false, false]);

    // 記号類
    mapping.insert('、', [false, false, true, false, false, false]);
    mapping.insert('。', [false, false, true, false, false, true]);
    // 伸ばし棒を追加
    mapping.insert('ー', [false, false, false, false, false, true]);
    mapping.insert('～', [false, false, false, false, false, true]); // 波ダッシュも同じ点字パターン

    // その他の記号類
    mapping.insert('？', [false, false, true, true, false, true]);
    mapping.insert('！', [false, false, true, true, true, false]);
    mapping.insert('「', [false, false, true, true, false, false]);
    mapping.insert('」', [false, false, false, true, true, false]);

    // 濁音・半濁音の印
    mapping.insert('゛', [false, false, false, false, true, true]); // 濁点
    mapping.insert('゜', [false, false, false, true, true, true]); // 半濁点

    // 濁音
    mapping.insert('が', [true, false, true, false, false, true]);
    mapping.insert('ぎ', [true, false, true, true, false, true]);
    mapping.insert('ぐ', [true, true, true, false, false, true]);
    mapping.insert('げ', [true, true, true, true, false, true]);
    mapping.insert('ご', [true, false, true, true, true, true]);

    mapping.insert('ざ', [true, false, false, false, true, true]);
    mapping.insert('じ', [true, false, false, true, true, true]);
    mapping.insert('ず', [true, true, false, false, true, true]);
    mapping.insert('ぜ', [true, true, false, true, true, true]);
    mapping.insert('ぞ', [true, false, false, true, true, true]);

    mapping.insert('だ', [true, false, true, false, true, true]);
    mapping.insert('ぢ', [true, false, true, true, true, true]);
    mapping.insert('づ', [true, true, true, false, true, true]);
    mapping.insert('で', [true, true, true, true, true, true]);
    mapping.insert('ど', [true, false, true, true, true, true]);

    mapping.insert('ば', [false, true, true, false, false, true]);
    mapping.insert('び', [false, true, true, true, false, true]);
    mapping.insert('ぶ', [false, true, true, false, true, true]);
    mapping.insert('べ', [false, true, true, true, true, true]);
    mapping.insert('ぼ', [false, true, true, true, true, true]);

    // 半濁音
    mapping.insert('ぱ', [false, true, true, false, false, true]);
    mapping.insert('ぴ', [false, true, true, true, false, true]);
    mapping.insert('ぷ', [false, true, true, false, true, true]);
    mapping.insert('ぺ', [false, true, true, true, true, true]);
    mapping.insert('ぽ', [false, true, true, true, true, true]);

    // 小さい文字
    mapping.insert('ぁ', [true, false, false, false, false, false]);
    mapping.insert('ぃ', [true, false, false, true, false, false]);
    mapping.insert('ぅ', [true, true, false, false, false, false]);
    mapping.insert('ぇ', [true, true, false, true, false, false]);
    mapping.insert('ぉ', [true, false, false, true, true, false]);
    mapping.insert('っ', [true, true, true, false, true, false]);
    mapping.insert('ゃ', [false, true, false, false, false, true]);
    mapping.insert('ゅ', [false, true, true, false, false, true]);
    mapping.insert('ょ', [false, true, false, true, false, true]);

    // 数字
    mapping.insert('0', [true, false, true, false, false, false]);
    mapping.insert('1', [true, false, false, false, false, false]);
    mapping.insert('2', [true, false, false, true, false, false]);
    mapping.insert('3', [true, true, false, false, false, false]);
    mapping.insert('4', [true, true, false, true, false, false]);
    mapping.insert('5', [true, false, false, true, true, false]);
    mapping.insert('6', [true, true, true, false, false, false]);
    mapping.insert('7', [true, true, true, true, false, false]);
    mapping.insert('8', [true, false, true, true, true, false]);
    mapping.insert('9', [false, true, true, false, false, false]);

    mapping
}

// 数字かどうかをチェックする関数
pub fn is_number(c: char) -> bool {
    c.is_ascii_digit()
}

// 数符のパターンを返す関数
pub fn get_number_prefix() -> [bool; 6] {
    [false, false, true, true, true, true]
}
