mod opt;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let input = std::env::args().nth(1).unwrap_or(".".to_owned());
    let input = fs::canonicalize(&input).unwrap();
    let output = std::env::args()
        .nth(2)
        .unwrap_or(input.to_str().unwrap().to_owned());
    let output = PathBuf::from(output);
    println!("{:?} => {:?}", input, output);
    let mut entries = fs::read_dir(input)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|p| {
            p.as_ref()
                .unwrap()
                .to_str()
                .to_owned()
                .unwrap()
                .ends_with("jpg")
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    entries.sort();

    let mut index = 1;
    for i in 1..entries.len() {
        let img1 = image::open(&entries[i - 1]).unwrap();
        let img2 = image::open(&entries[i]).unwrap();
        if let Some(pic) = opt::join::try_join(img1, img2) {
            pic.save("C:\\Users\\zhangxu\\Desktop\\MERGED.jpg").unwrap();
        }
        index += 1;
    }
}
