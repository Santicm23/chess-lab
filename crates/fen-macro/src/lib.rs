use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn fen(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);

    match validate_fen(&lit) {
        Err(e) => return e.into_compile_error().into(),
        Ok(()) => quote! { #lit }.into(),
    }
}

fn validate_fen(lit: &LitStr) -> syn::Result<()> {
    let fen = lit.value();
    let parts: Vec<&str> = fen.split(' ').collect();

    if parts.len() != 6 {
        return Err(err(
            lit,
            format!(
                "FEN must have 6 space-separated fields, found {}.\n\
                 Expected: <pieces> <turn> <castling> <en-passant> <halfmove> <fullmove>",
                parts.len()
            ),
        ));
    }

    validate_pieces(lit, parts[0])?;
    validate_turn(lit, parts[1])?;
    validate_castling(lit, parts[2])?;
    validate_en_passant(lit, parts[3])?;
    validate_halfmove(lit, parts[4])?;
    validate_fullmove(lit, parts[5])?;
    Ok(())
}

fn validate_pieces(lit: &LitStr, placement: &str) -> syn::Result<()> {
    let ranks: Vec<&str> = placement.split('/').collect();

    if ranks.len() != 8 {
        return Err(err(
            lit,
            format!(
                "piece placement must have 8 ranks separated by '/', found {}",
                ranks.len()
            ),
        ));
    }

    let mut white_kings = 0u32;
    let mut black_kings = 0u32;

    for (rank_idx, rank) in ranks.iter().enumerate() {
        let rank_num = 8 - rank_idx; // FEN rank 8 is index 0
        let mut file_count = 0u32;

        for ch in rank.chars() {
            match ch {
                '1'..='8' => {
                    file_count += ch.to_digit(10).unwrap();
                }
                'P' | 'N' | 'B' | 'R' | 'Q' => file_count += 1,
                'p' | 'n' | 'b' | 'r' | 'q' => file_count += 1,
                'K' => {
                    white_kings += 1;
                    file_count += 1;
                }
                'k' => {
                    black_kings += 1;
                    file_count += 1;
                }
                _ => {
                    return Err(err(
                        lit,
                        format!("invalid character '{ch}' in rank {rank_num} ('{rank}')"),
                    ));
                }
            }
        }

        if file_count != 8 {
            return Err(err(
                lit,
                format!("rank {rank_num} ('{rank}') has {file_count} squares, expected 8"),
            ));
        }
    }

    if white_kings != 1 {
        return Err(err(
            lit,
            format!("position must have exactly 1 white king ('K'), found {white_kings}"),
        ));
    }
    if black_kings != 1 {
        return Err(err(
            lit,
            format!("position must have exactly 1 black king ('k'), found {black_kings}"),
        ));
    }
    Ok(())
}

fn validate_turn(lit: &LitStr, turn: &str) -> syn::Result<()> {
    match turn {
        "w" | "b" => Ok(()),
        _ => Err(err(
            lit,
            format!("active color must be 'w' or 'b', got '{turn}'"),
        )),
    }
}

fn validate_castling(lit: &LitStr, castling: &str) -> syn::Result<()> {
    if castling == "-" {
        return Ok(());
    } else if castling.is_empty() {
        return Err(err(
            lit,
            "castling rights cannot be empty — use '-' for no rights",
        ));
    }
    // Must be some subset of KQkq in that order, no duplicates
    let valid_chars = ['K', 'Q', 'k', 'q'];
    let mut seen = [false; 4];

    for ch in castling.chars() {
        match valid_chars.iter().position(|&c| c == ch) {
            Some(i) if seen[i] => {
                return Err(err(
                    lit,
                    format!("duplicate castling right '{ch}' in '{castling}'"),
                ));
            }
            Some(i) => seen[i] = true,
            None => {
                return Err(err(
                    lit,
                    format!("invalid castling character '{ch}' — must be K, Q, k, q, or '-'"),
                ));
            }
        }
    }
    Ok(())
}

fn validate_en_passant(lit: &LitStr, ep: &str) -> syn::Result<()> {
    if ep == "-" {
        return Ok(());
    }
    let chars: Vec<char> = ep.chars().collect();
    if chars.len() != 2 {
        return Err(err(
            lit,
            format!("en passant square must be like 'e3' or '-', got '{ep}'"),
        ));
    }
    let (file, rank) = (chars[0], chars[1]);
    if !('a'..='h').contains(&file) {
        return Err(err(
            lit,
            format!("en passant file must be a–h, got '{file}'"),
        ));
    }
    // En passant can only be on rank 3 (black just moved) or rank 6 (white just moved)
    if rank != '3' && rank != '6' {
        return Err(err(
            lit,
            format!("en passant rank must be 3 or 6, got '{rank}'"),
        ));
    }
    Ok(())
}

fn validate_halfmove(lit: &LitStr, hm: &str) -> syn::Result<()> {
    match hm.parse::<u32>() {
        Ok(n) if n > 150 => Err(err(
            lit,
            format!("halfmove clock is {n}, which exceeds 150 (the 75-move rule ceiling)"),
        )),
        Err(_) => Err(err(
            lit,
            format!("halfmove clock must be a non-negative integer, got '{hm}'"),
        )),
        Ok(_) => Ok(()),
    }
}

fn validate_fullmove(lit: &LitStr, fm: &str) -> syn::Result<()> {
    match fm.parse::<u32>() {
        Ok(0) | Err(_) => Err(err(
            lit,
            "fullmove number must be a positive non-zero integer, got '{fm}'",
        )),
        Ok(_) => Ok(()),
    }
}

fn err(lit: &LitStr, msg: impl std::fmt::Display) -> syn::Error {
    syn::Error::new_spanned(lit, msg)
}
