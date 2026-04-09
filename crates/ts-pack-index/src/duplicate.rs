use std::collections::HashSet;

pub(crate) fn tokenize_normalized(source: &[u8]) -> Vec<u64> {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut tokens = Vec::new();
    let mut i = 0;
    while i < source.len() {
        let b = source[i];
        if (b as char).is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if (b as char).is_ascii_alphabetic() || b == b'_' {
            let mut j = i + 1;
            while j < source.len() {
                let c = source[j];
                if (c as char).is_ascii_alphanumeric() || c == b'_' {
                    j += 1;
                } else {
                    break;
                }
            }
            let mut h = FNV_OFFSET;
            for ch in b"<id>" {
                h ^= *ch as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            tokens.push(h);
            i = j;
            continue;
        }
        if (b as char).is_ascii_digit() {
            let mut j = i + 1;
            while j < source.len() {
                let c = source[j];
                if (c as char).is_ascii_digit() {
                    j += 1;
                } else {
                    break;
                }
            }
            let mut h = FNV_OFFSET;
            for ch in b"<num>" {
                h ^= *ch as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            tokens.push(h);
            i = j;
            continue;
        }

        let punct = match b {
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',' | b'.' | b':' | b'+' | b'-' | b'*' | b'/' | b'%'
            | b'<' | b'>' | b'=' => Some(b),
            _ => None,
        };
        if let Some(p) = punct {
            let mut h = FNV_OFFSET;
            h ^= p as u64;
            h = h.wrapping_mul(FNV_PRIME);
            tokens.push(h);
            i += 1;
            continue;
        }

        i += 1;
    }
    tokens
}

pub(crate) fn winnow_fingerprints(tokens: &[u64], k: usize, window: usize) -> HashSet<u64> {
    if tokens.len() < k {
        return HashSet::new();
    }
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hashes = Vec::new();
    for i in 0..=tokens.len() - k {
        let mut h = FNV_OFFSET;
        for t in &tokens[i..i + k] {
            h ^= *t;
            h = h.wrapping_mul(FNV_PRIME);
        }
        hashes.push(h);
    }
    if hashes.is_empty() {
        return HashSet::new();
    }
    if hashes.len() <= window {
        return [*hashes.iter().min().unwrap()].into_iter().collect();
    }
    let mut fps = HashSet::new();
    for i in 0..=hashes.len() - window {
        let mut min = hashes[i];
        for value in hashes.iter().skip(i).take(window) {
            if *value < min {
                min = *value;
            }
        }
        fps.insert(min);
    }
    fps
}

pub(crate) fn kgram_hashes(tokens: &[u64], k: usize) -> HashSet<u64> {
    if tokens.len() < k {
        return HashSet::new();
    }
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut out = HashSet::new();
    for i in 0..=tokens.len() - k {
        let mut h = FNV_OFFSET;
        for t in &tokens[i..i + k] {
            h ^= *t;
            h = h.wrapping_mul(FNV_PRIME);
        }
        out.insert(h);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{kgram_hashes, tokenize_normalized, winnow_fingerprints};

    #[test]
    fn normalize_identifiers_and_numbers() {
        let tokens = tokenize_normalized(b"count = total + 42");
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], tokens[2]);
        assert_ne!(tokens[0], tokens[3]);
    }

    #[test]
    fn winnow_is_stable_on_repeated_sequence() {
        let tokens = tokenize_normalized(b"count = total + 42");
        let repeated = tokens
            .iter()
            .copied()
            .cycle()
            .take(tokens.len() * 3)
            .collect::<Vec<_>>();
        let fps = winnow_fingerprints(&repeated, 3, 2);
        assert!(!fps.is_empty());
    }

    #[test]
    fn kgrams_exist_for_short_token_sequences() {
        let tokens = tokenize_normalized(b"foo(bar)");
        let grams = kgram_hashes(&tokens, 3);
        assert!(!grams.is_empty());
    }
}
