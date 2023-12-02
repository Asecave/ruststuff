use rustmatica::Litematic;
use rustmatica::fastnbt::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = Litematic::read_file("/home/asecave/git/ruststuff/src/temp.litematic")?;

    for t in build.regions[0].tile_entities.iter() {
        for (k, v) in t.properties.iter() {
            if k == "Items" {
                if let Value::List(vector) = v {
                    for i in vector.iter() {
                        println!("{:?}", i);
                    }
                }
            }
        }
    }

    Ok(())
}
