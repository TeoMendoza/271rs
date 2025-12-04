use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

type FileLines = Vec<String>;
type FileMap = HashMap<String, FileLines>;

#[derive(Serialize, Deserialize)]
struct ScmCommit {
    Init: FileMap,
    Diff: HashMap<String, Vec<String>>,
    Delete: FileMap,
}

#[derive(Serialize, Deserialize)]
struct ScmState {
    Latest: FileMap,
    Commit: Vec<ScmCommit>,
}

fn main() {
    let Arguments: Vec<String> = env::args().collect();
    if Arguments.len() != 2 {
        eprintln!("usage: scm <commit|revert>");
        std::process::exit(1);
    }

    let Command = &Arguments[1];

    match Command.as_str() {
        "commit" => RunCommit(),
        "revert" => RunRevert(),
        _ => {
            eprintln!("unknown command {}", Command);
            std::process::exit(1);
        }
    }
}

fn RunCommit() {
    let ScmPath = PathBuf::from(".scm");
    let WorkingFiles = ReadWorkingFiles();

    let ScmStateOption = LoadScmState(&ScmPath);

    match ScmStateOption {
        None => {
            let Latest = WorkingFiles;
            let Init = Latest.clone();
            let Diff = HashMap::new();
            let Delete = FileMap::new();

            let FirstCommit = ScmCommit { Init, Diff, Delete };
            let Commit = vec![FirstCommit];

            let State = ScmState { Latest, Commit };
            SaveScmState(&ScmPath, &State);
        }
        Some(mut State) => {
            if State.Commit.is_empty() {
                eprintln!("invalid .scm file: no commits");
                std::process::exit(1);
            }

            let LatestBefore = State.Latest.clone();

            let OldFileNames = CollectOldFileNames(&LatestBefore);
            let NewFileNames = CollectNewFileNames(&WorkingFiles, &OldFileNames);

            let mut Init = FileMap::new();
            for Name in NewFileNames {
                if let Some(Lines) = WorkingFiles.get(&Name) {
                    Init.insert(Name.clone(), Lines.clone());
                }
            }

            let mut Diff = HashMap::new();
            let mut Delete = FileMap::new();

            for Name in OldFileNames {
                let BeforeLinesOption = LatestBefore.get(&Name);
                let AfterLinesOption = WorkingFiles.get(&Name);

                match (BeforeLinesOption, AfterLinesOption) {
                    (Some(BeforeLines), Some(AfterLines)) => {
                        if BeforeLines != AfterLines {
                            let DiffForFile = DiffLines(BeforeLines, AfterLines);
                            if !DiffForFile.is_empty() {
                                Diff.insert(Name.clone(), DiffForFile);
                            }
                        }
                    }
                    (Some(BeforeLines), None) => {
                        Delete.insert(Name.clone(), BeforeLines.clone());
                    }
                    _ => {}
                }
            }

            State.Latest = WorkingFiles;
            State.Commit.push(ScmCommit { Init, Diff, Delete });

            SaveScmState(&ScmPath, &State);
        }
    }
}

fn RunRevert() {
    let ScmPath = PathBuf::from(".scm");
    let ScmStateOption = LoadScmState(&ScmPath);

    let mut State = match ScmStateOption {
        None => {
            eprintln!("no .scm file to revert");
            std::process::exit(1);
        }
        Some(State) => State,
    };

    if State.Commit.len() < 2 {
        eprintln!("no earlier commit to revert to");
        std::process::exit(1);
    }

    WriteLatestToDisk(&State.Latest);

    let LastCommitOption = State.Commit.last();
    if LastCommitOption.is_none() {
        eprintln!("invalid .scm file: no commits");
        std::process::exit(1);
    }
    let LastCommit = LastCommitOption.unwrap();

    for (Name, DiffLinesForFile) in LastCommit.Diff.iter() {
        let PathValue = PathBuf::from(Name);
        let CurrentLines = ReadFileLines(&PathValue);
        let PreviousLines = ApplyReversePatch(&CurrentLines, DiffLinesForFile);
        WriteFileLines(&PathValue, &PreviousLines);
    }

    for (Name, Lines) in LastCommit.Delete.iter() {
        let PathValue = PathBuf::from(Name);
        WriteFileLines(&PathValue, Lines);
    }

    for Name in LastCommit.Init.keys() {
        let PathValue = PathBuf::from(Name);
        let _ = fs::remove_file(&PathValue);
    }

    State.Commit.pop();

    let NewLatest = ReadWorkingFiles();
    State.Latest = NewLatest;

    SaveScmState(&ScmPath, &State);
}

fn ReadWorkingFiles() -> FileMap {
    let mut Result = FileMap::new();
    let Start = PathBuf::from(".");
    let RelativeRoot = PathBuf::new();

    ReadWorkingFilesRecursive(&Start, &mut Result, &RelativeRoot);

    Result
}

fn ReadWorkingFilesRecursive(Current: &PathBuf, Result: &mut FileMap, Relative: &PathBuf) {
    let ReadDirResult = fs::read_dir(Current);
    if ReadDirResult.is_err() {
        return;
    }

    for EntryResult in ReadDirResult.unwrap() {
        if EntryResult.is_err() {
            continue;
        }

        let Entry = EntryResult.unwrap();

        let FileTypeResult = Entry.file_type();
        if FileTypeResult.is_err() {
            continue;
        }
        let FileType = FileTypeResult.unwrap();

        let NameOs = Entry.file_name();
        let Name = match NameOs.to_str() {
            None => continue,
            Some(Name) => Name.to_string(),
        };

        if Name.starts_with('.') {
            continue;
        }

        let mut MutRelative = Relative.clone();
        MutRelative.push(&Name);

        if FileType.is_dir() {
            let mut MutCurrent = Current.clone();
            MutCurrent.push(&Name);
            ReadWorkingFilesRecursive(&MutCurrent, Result, &MutRelative);
        } else if FileType.is_file() {
            let Lines = ReadFileLines(&MutRelative);
            let Key = MutRelative.to_string_lossy().to_string();
            Result.insert(Key, Lines);
        }
    }
}

fn ReadFileLines(PathValue: &Path) -> FileLines {
    let Data = fs::read_to_string(PathValue).unwrap_or_else(|_| String::new());
    let mut Lines: Vec<String> = Data.split('\n').map(|Line| Line.to_string()).collect();
    if Lines.len() == 1 && Lines[0].is_empty() {
        Lines.clear();
    }
    Lines
}

fn WriteFileLines(PathValue: &Path, Lines: &FileLines) {
    let Content = Lines.join("\n");
    if let Some(Parent) = PathValue.parent() {
        let _ = fs::create_dir_all(Parent);
    }
    let _ = fs::write(PathValue, Content);
}

fn WriteLatestToDisk(Latest: &FileMap) {
    for (Name, Lines) in Latest.iter() {
        let PathValue = PathBuf::from(Name);
        WriteFileLines(&PathValue, Lines);
    }
}

fn LoadScmState(ScmPath: &Path) -> Option<ScmState> {
    let Metadata = fs::metadata(ScmPath).ok()?;
    if Metadata.len() == 0 {
        return None;
    }
    let Data = fs::read_to_string(ScmPath).ok()?;
    if Data.trim().is_empty() {
        return None;
    }
    let Parsed = serde_json::from_str::<ScmState>(&Data).ok()?;
    Some(Parsed)
}

fn SaveScmState(ScmPath: &Path, State: &ScmState) {
    let Data = serde_json::to_string_pretty(State).unwrap();
    let _ = fs::write(ScmPath, Data);
}

fn CollectOldFileNames(Latest: &FileMap) -> Vec<String> {
    Latest.keys().cloned().collect()
}

fn CollectNewFileNames(WorkingFiles: &FileMap, OldFileNames: &Vec<String>) -> Vec<String> {
    let mut Names = Vec::new();
    for Name in WorkingFiles.keys() {
        if !OldFileNames.contains(Name) {
            Names.push(Name.to_string());
        }
    }
    Names
}

fn DiffLines(Before: &FileLines, After: &FileLines) -> Vec<String> {
    let OldLines = After;
    let NewLines = Before;

    let N = OldLines.len();
    let M = NewLines.len();

    let mut Dp: Vec<Vec<usize>> = vec![vec![0; M + 1]; N + 1];

    let mut I = N;
    while I > 0 {
        I -= 1;
        let mut J = M;
        while J > 0 {
            J -= 1;
            if OldLines[I] == NewLines[J] {
                Dp[I][J] = Dp[I + 1][J + 1] + 1;
            } else {
                let Down = Dp[I + 1][J];
                let Right = Dp[I][J + 1];
                Dp[I][J] = if Down >= Right { Down } else { Right };
            }
        }
    }

    enum PatchOp {
        Delete(usize),
        Insert(usize, String),
    }

    let mut Ops: Vec<PatchOp> = Vec::new();

    let mut IBack = N;
    let mut JBack = M;

    while IBack > 0 || JBack > 0 {
        if IBack > 0 && JBack > 0 && OldLines[IBack - 1] == NewLines[JBack - 1] {
            IBack -= 1;
            JBack -= 1;
        } else if JBack > 0 && (IBack == 0 || Dp[IBack][JBack - 1] >= Dp[IBack - 1][JBack]) {
            let Line = NewLines[JBack - 1].clone();
            Ops.push(PatchOp::Insert(IBack, Line));
            JBack -= 1;
        } else if IBack > 0 {
            Ops.push(PatchOp::Delete(IBack - 1));
            IBack -= 1;
        } else {
            break;
        }
    }

    let mut Result: Vec<String> = Vec::new();

    for Op in Ops {
        match Op {
            PatchOp::Delete(Index) => {
                Result.push(format!("D {}", Index));
            }
            PatchOp::Insert(Index, Line) => {
                Result.push(format!("I {} {}", Index, Line));
            }
        }
    }

    Result
}

fn ApplyReversePatch(Current: &FileLines, DiffLinesForFile: &Vec<String>) -> FileLines {
    let mut Result = Current.clone();

    for OpLine in DiffLinesForFile {
        if OpLine.is_empty() {
            continue;
        }

        let mut Parts = OpLine.splitn(3, ' ');
        let Kind = Parts.next().unwrap_or("");
        let IndexText = Parts.next().unwrap_or("0");

        let ParsedIndex = IndexText.parse::<usize>();
        if ParsedIndex.is_err() {
            continue;
        }
        let Index = ParsedIndex.unwrap();

        if Kind == "D" {
            if Index < Result.len() {
                Result.remove(Index);
            }
        } else if Kind == "I" {
            let LineContent = Parts.next().unwrap_or("").to_string();
            if Index <= Result.len() {
                Result.insert(Index, LineContent);
            } else {
                Result.push(LineContent);
            }
        }
    }

    Result
}
