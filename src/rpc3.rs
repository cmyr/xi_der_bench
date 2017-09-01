//! A coarse implementation of what this would look like if we were just
//! borrowing directly.


use rpc2;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EmptyStruct {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum CoreNotification<'a> {
    Edit(EditNotification<'a>),
    Plugin(rpc2::PluginNotification),
    CloseView { view_id: &'a str },
    Save { view_id: &'a str, file_path: &'a str },
    SetTheme { theme_name: &'a str },
    ClientStarted(EmptyStruct),
    NewView { file_path: Option<&'a str> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
//pub struct InsertParams<'a> { chars: &'a str }
pub struct InsertParams { chars: String }
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestLinesParams(usize, usize);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ClickParams(usize, usize, usize, usize);
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DragParams(usize, usize, usize);


#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method")]
pub enum EditNotification<'a> {
    Insert { view_id: &'a str, params: InsertParams },
    RequestLines { view_id: &'a str, params: RequestLinesParams },
    Scroll { view_id: &'a str, params: RequestLinesParams },
    MoveWordRight { view_id: &'a str },
    MoveWordLeft { view_id: &'a str },
    DeleteForward { view_id: &'a str },
    DeleteBackward { view_id: &'a str },
    InsertNewline { view_id: &'a str },
    Click { view_id: &'a str, params: ClickParams },
    Drag { view_id: &'a str, params: DragParams },
    DeleteWordForward,
    DeleteWordBackward,
    DeleteToEndOfParagraph,
    DeleteToBeginningOfLine,
    MoveUp,
    MoveUpAndModifySelection,
    MoveDown,
    MoveDownAndModifySelection,
    MoveLeft,
    MoveLeftAndModifySelection,
    MoveRight,
    MoveRightAndModifySelection,
    MoveWordLeftAndModifySelection,
    MoveWordRightAndModifySelection,
    MoveToBeginningOfParagraph,
    MoveToEndOfParagraph,
    MoveToLeftEndOfLine,
    MoveToLeftEndOfLineAndModifySelection,
    MoveToRightEndOfLine,
    MoveToRightEndOfLineAndModifySelection,
    MoveToBeginningOfDocument,
    MoveToBeginningOfDocumentAndModifySelection,
    MoveToEndOfDocument,
    MoveToEndOfDocumentAndModifySelection,
    ScrollPageUp,
    PageUpAndModifySelection,
    ScrollPageDown,
    PageDownAndModifySelection,
    SelectAll,
    AddSelectionAbove,
    AddSelectionBelow,
    GotoLine { line: u64 },
    Yank,
    Transpose,
    Gesture { line: u64, column: u64, ty: rpc2::GestureType},
    Undo,
    Redo,
    FindNext { wrap_around: bool, allow_same: bool },
    FindPrevious { wrap_around: bool },
    DebugRewrap,
    DebugPrintSpans,
}
