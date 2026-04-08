# Create a new project called myproject
cargo new myproject

# Change to project directory
cd myproject

# Show project directory structure
tree
# Output: 
# .
# ├── Cargo.toml
# └── src
#     └── main.rs

# Build the project  
cargo build

# Run the project
cargo run
# Output: Hello, world!

# Add dependencies
cargo add serde

# Upgrade dependencies
cargo upgrade

# Show project directory structure now including added dependencies
tree 
# Output:
# .
# ├── Cargo.lock
# ├── Cargo.toml
# └── src
#     └── main.rs