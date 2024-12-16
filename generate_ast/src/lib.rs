use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct TreeType {
    struct_name: String,
    fields: Vec<String>,
}

pub fn define_ast(
    output_dir: &str,
    base_name: String,
    tree_types: &[String],
    require_hash: bool,
) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path).expect("Failed to create file on specified location");

    writeln!(&mut file, "use crate::token::*;")?;
    writeln!(&mut file, "use crate::object::*;")?;
    if base_name.to_lowercase().contains("stmt") {
        writeln!(&mut file, "use crate::expr::*;")?;
    }
    if require_hash {
        writeln!(&mut file, "use std::hash::Hash;")?;
    }
    writeln!(&mut file)?;

    // define Visitor trait
    define_visitor(&mut file, &base_name, tree_types)?;

    // define Base
    define_base(&mut file, &base_name, tree_types)?;
    writeln!(&mut file)?;

    for tree_type in tree_types {
        // Get the tree name and the fields
        let (tree_name, fields) = tree_type.split_once(':').unwrap();
        // Struct name
        let struct_name = tree_name.trim().to_string();
        // Struct fields
        let field_vec: Vec<String> = fields.split(',').map(str::to_string).collect();
        // Define Struct
        define_type(
            &mut file,
            &base_name.trim(),
            TreeType {
                struct_name,
                fields: field_vec,
            },
        )?
    }

    // Implement Base Type
    impl_base_type(&mut file, &base_name, tree_types, require_hash)?;
    if require_hash {
        impl_partial_eq_hash(&mut file, &base_name)?;
    }
    writeln!(&mut file)?;

    Ok(())
}

fn define_visitor(file: &mut File, base_name: &str, tree_types: &[String]) -> io::Result<()> {
    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;
    for tree_type in tree_types {
        let (tree_name, _) = tree_type.split_once(':').unwrap();
        writeln!(
            file,
            "    fn visit_{}_{}(&mut self, {}: &{}{}) -> T;",
            tree_name.trim().to_lowercase(),
            base_name.trim().to_lowercase(),
            base_name.trim().to_lowercase(),
            tree_name.trim(),
            base_name.trim(),
        )?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

fn define_base(file: &mut File, base_name: &str, tree_types: &[String]) -> io::Result<()> {
    writeln!(file, "#[derive(Debug, Clone)]")?;
    writeln!(file, "pub enum {} {{", base_name)?;
    for tree_type in tree_types {
        let (tree_name, _) = tree_type.split_once(':').unwrap();
        writeln!(
            file,
            "    {}({}{}),",
            tree_name.trim(),
            tree_name.trim(),
            base_name
        )?;
    }
    writeln!(file, "}}")?;
    Ok(())
}

fn define_type(file: &mut File, base_name: &str, tree_type: TreeType) -> io::Result<()> {
    // Define Struct type
    writeln!(file, "#[derive(Debug, Clone)]")?;
    writeln!(file, "pub struct {}{} {{", tree_type.struct_name, base_name)?;
    for field in tree_type.fields {
        let (field_type, field_name) = field.trim().split_once(' ').unwrap();
        writeln!(file, "    pub {}: {},", field_name, field_type)?;
    }

    writeln!(file, "}}",)?;
    writeln!(file)?;
    Ok(())
}

fn impl_base_type(
    file: &mut File,
    base_name: &str,
    tree_types: &[String],
    require_hash: bool,
) -> io::Result<()> {
    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> T {{",
        base_name
    )?;
    writeln!(file, "        match self {{",)?;
    for tree_type in tree_types.iter() {
        let (tree_name, _) = tree_type.split_once(':').unwrap();
        writeln!(
            file,
            "            {}::{}({}_{}) => visitor.visit_{}_{}({}_{}),",
            base_name.trim(),
            tree_name.trim(),
            tree_name.trim().to_lowercase(),
            base_name.trim().to_lowercase(),
            tree_name.trim().to_lowercase(),
            base_name.trim().to_lowercase(),
            tree_name.trim().to_lowercase(),
            base_name.trim().to_lowercase(),
        )?;
    }
    writeln!(file, "        }}",)?;
    writeln!(file, "    }}")?;
    if require_hash {
        writeln!(file, "    fn get_uid(&self) -> usize {{")?;
        writeln!(file, "        match self {{",)?;
        for tree_type in tree_types.iter() {
            let (tree_name, _) = tree_type.split_once(':').unwrap();
            writeln!(
                file,
                "            {}::{}(expr) => expr.uid,",
                base_name.trim(),
                tree_name.trim(),
            )?;
        }
        writeln!(file, "        }}",)?;
        writeln!(file, "    }}")?;
    }
    writeln!(file, "}}",)?;
    writeln!(file)?;
    Ok(())
}

fn impl_partial_eq_hash(file: &mut File, base_name: &str) -> io::Result<()> {
    writeln!(file, "impl PartialEq for {} {{", base_name)?;
    writeln!(file, "    fn eq(&self, other: &Self) -> bool {{")?;
    writeln!(file, "        self.get_uid() == other.get_uid()",)?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl Eq for {} {{}}", base_name)?;
    writeln!(file)?;

    writeln!(file, "impl Hash for {} {{", base_name)?;
    writeln!(
        file,
        "    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {{"
    )?;
    writeln!(file, "        self.get_uid().hash(state);",)?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    Ok(())
}
