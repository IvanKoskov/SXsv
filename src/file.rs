/*
File_sxsv structure holds a state of the opened document. It persists always and should be
a point where any editor starts. It should always be updated.
*/

pub struct File_sxsv {
    extension: String,
    path: String,
    contents: String,
    log: String,
    full_name: String,
}
