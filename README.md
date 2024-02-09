# Seam carving

### About the project
##### Fast, robust and useful: this is not about my project. It's straightforward and just works. I had no intention to make it best of the best, but rather to discover the algorithm to myself.

### Results
![Demo](https://github.com/mipselqq/seam-carving/blob/main/images/illustration.jpg?raw=true "Demo")

### Example usage
Fast algorithm, relative target dimensions
```rust
cargo run --release -- -i tower.jpg -w 50% -h 50% -o carved.png -f
```
Precise algorithm, absolute target dimensions
```rust
cargo run --release -- -i tower.jpg -w 1000 -h 800 -o carved.png
```
