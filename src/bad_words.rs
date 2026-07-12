use regex::Regex;
use sqlx::MySqlPool;
use std::sync::OnceLock;

static BAD_WORD_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();

#[derive(Debug, sqlx::FromRow)]
struct BadWordMinimal {
    sWord: String,
}



pub async fn setup_bad_words_library(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let rows: Vec<BadWordMinimal> = sqlx::query_as!(BadWordMinimal, "SELECT sWord FROM BadWords")
        .fetch_all(pool)
        .await?;

    let patterns = rows
        .into_iter()
        .filter_map(|row| Some(row.sWord))
        .map(|word| {
            let word = word.trim().to_lowercase();
            let escaped = regex::escape(&word).replace(r"\*", "[A-Za-z0-9]*");
            Regex::new(&format!(r"\b{}\b", escaped)).unwrap()
        })
        .collect::<Vec<_>>();

    BAD_WORD_PATTERNS.set(patterns).ok();
    Ok(())
}

fn get_profanity_replacement_word(word: &str) -> String {
    if word.len() > 1 {
        format!(
            "{}{}{}",
            &word[0..1],
            "*".repeat(word.len() - 2),
            &word[word.len() - 1..]
        )
    } else {
        "*".to_string()
    }
}

pub fn replace_bad_words(text: &str) -> String {
    let Some(patterns) = BAD_WORD_PATTERNS.get() else {
        return text.to_string(); // Not initialized
    };

    let mut result = text.to_string();
    let words: Vec<&str> = regex::Regex::new(r"[^\w]+").unwrap().split(text).collect();

    for word in words {
        if word.is_empty() {
            continue;
        }

        if patterns.iter().any(|re| re.is_match(&word.to_lowercase())) {
            let replacement = get_profanity_replacement_word(word);
            result = Regex::new(&format!(r"\b{}\b", regex::escape(word)))
                .unwrap()
                .replace_all(&result, &replacement)
                .to_string();
        }
    }

    result
}
