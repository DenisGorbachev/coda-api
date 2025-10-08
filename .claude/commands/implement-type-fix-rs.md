Implement `src/bin/replace_type.rs`

* Use Clap V4 derive
* Make a `Cli` struct
  * Fields
    * `name: String` /// The type name
    * `replacement_path: PathBuf` /// The file that contains the replacement for the type
    * `input_path: PathBuf` /// The path to the file with the type
  * Functions
    * `pub fn run(self)`
      * Tests
        * Outputs
* Tests
  * Preserves the comment 
