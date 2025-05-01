use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use serde::Serialize;

// CCFinderSWの出力ファイルの各ファイル情報を表す構造体
#[derive(Debug, Serialize)]
pub struct FileData {
    file_id: isize,      // ファイルID
    file_path: String,   // ファイルパス
    loc: isize,          // 行数
    token_count: isize,  // トークン数
}

// クローンセットを表す構造体
#[derive(Debug, Serialize)]
pub struct CloneSet {
    clone_id: isize,     // クローンセットID
    fragments: Vec<Fragment>,  // クローンフラグメントのリスト
}

// クローンフラグメントを表す構造体
#[derive(Debug, Serialize)]
pub struct Fragment {
    file_id: isize,      // ファイルID
    start_line: isize,   // 開始行
    start_col: isize,    // 開始列
    end_line: isize,     // 終了行
    end_col: isize,      // 終了列
}

// CCFinderSWの出力全体を表す構造体
#[derive(Debug, Serialize)]
pub struct CCFinderOutput {
    file_data: Vec<FileData>,    // ファイル情報のリスト
    clone_sets: Vec<CloneSet>,   // クローンセットのリスト
}

// CCFinderSWの出力ファイルをパースする関数
pub fn parse_ccfinder_output(input: PathBuf) -> Result<CCFinderOutput>{
    // 入力ファイルを開く
    let file = File::open(&input)?;
    let reader = BufReader::new(file);

    // パース結果を格納する変数
    let mut file_data = Vec::new();      // ファイル情報のリスト
    let mut clone_sets = Vec::new();     // クローンセットのリスト
    let mut mode = 0;                    // 現在のパースモード
    let mut current_clone_set = CloneSet { clone_id: -1, fragments: Vec::new() };  // 現在処理中のクローンセット

    // ファイルを1行ずつ処理
    for line in reader.lines() {
        let line = line?;
        
        // モードを設定する
        // 0: ヘッダー
        // 1: ソースファイル
        // 2: クローンセット
        if line.starts_with("#source_files") {
            mode = 1;
            continue;
        }
        if line.starts_with("#clone_sets") || line.starts_with("#clone_pairs") {
            mode = 2;
            continue;
        }

        // モードによって処理を分ける
        if mode == 0 { continue; }  // ヘッダー行はスキップ

        // ソースファイル行のパース
        if mode == 1 {
            if line.trim().is_empty() { continue; }
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 4 { continue; }  // 4つ未満ならスキップ
            let file_id = parts[0].parse::<isize>()?;
            let loc = parts[1].parse::<isize>()?;
            let token_count = parts[2].parse::<isize>()?;
            let file_path = parts[3].to_string();
            file_data.push(FileData { file_id, file_path, loc, token_count });
        }

        // クローンセット行のパース
        if mode == 2 {
            if line.trim().is_empty() { continue; }
            
            // クローンIDの行を処理
            if line.starts_with("cloneID:") {
                if let Some(id_str) = line.strip_prefix("cloneID:") {
                    let clone_id = id_str.trim().parse::<isize>()?;
                    // 直前のクローンセットを保存
                    if current_clone_set.clone_id != -1 {
                        clone_sets.push(current_clone_set);
                    }
                    current_clone_set = CloneSet { clone_id, fragments: Vec::new() };
                }
                continue;
            }

            // フラグメント行のパース
            let fragment_str = line.trim();
            let parts: Vec<&str> = fragment_str.split(':').collect();
            if parts.len() < 2 { continue; }
            let file_id = parts[0].parse::<isize>()?;
            let positions = parts[1].split(" - ").collect::<Vec<&str>>();
            if positions.len() < 2 { continue; }
            let start_position = positions[0].split(",").collect::<Vec<&str>>();
            let end_position = positions[1].split(",").collect::<Vec<&str>>();
            if start_position.len() < 2 || end_position.len() < 2 { continue; }
            let start_line = start_position[0].parse::<isize>()?;
            let start_col = start_position[1].parse::<isize>()?;
            let end_line = end_position[0].parse::<isize>()?;
            let end_col = end_position[1].parse::<isize>()?;

            let fragment = Fragment {
                file_id,
                start_line,
                start_col,
                end_line,
                end_col,
            };

            current_clone_set.fragments.push(fragment);
        }
    }

    // 最後のクローンセットを保存
    if current_clone_set.clone_id != -1 {
        clone_sets.push(current_clone_set);
    }

    Ok(CCFinderOutput { file_data, clone_sets })
}
