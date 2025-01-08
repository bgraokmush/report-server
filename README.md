<div id="top"></div>

<br />
<div align="center">
  <h3 align="center">Rust Report Server</h3>

  <p align="center">
    🚀 A lightweight report server application built using Rust, capable of loading and rendering data in JSON and text formats.
    <br />
    <a href="https://github.com/bgraokmush/report-server/issues">Report Bug</a>
    ·
    <a href="https://github.com/bgraokmush/report-server/issues">Request Feature</a>
  </p>
</div>

## 📌 About The Project

This project is a demonstration of a report server built with Rust. It can read data in both JSON and tabular (text) formats, process it using generic traits, and render the results with the Tera templating engine.

<p align="right">(<a href="#top">back to top</a>)</p>

### 🔧 Built With

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum/0.8.1/axum/)
- [Serde](https://serde.rs/)
- [Tera](https://docs.rs/tera/latest/tera/)
- [Tokio](https://tokio.rs/)

<p align="right">(<a href="#top">back to top</a>)</p>

## 🚀 Getting Started

To get a local copy up and running, follow these steps.

### Prerequisites

Ensure you have Rust and Cargo installed. You can install them via [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1. Clone the repo:
   ```sh
   git clone https://github.com/yourusername/rust-report-server.git
   ```
2. Navigate to the project directory:
   ```sh
   cd rust-report-server
   ```
3. Run the application:
   ```sh
   cargo run
   ```

<p align="right">(<a href="#top">back to top</a>)</p>

## 🔄 Usage

This project demonstrates the use of Rust traits for data loading and rendering reports. It includes two traits:

- `JsonLoader` for loading JSON data.
- `TabularLoader` for loading tabular text data.

Example report generation:

```rust
pub async fn generate_report<T: Serialize>(
    tera: Arc<Tera>,
    template_name: &str,
    context_data: T,
) -> Html<String> {
    let mut context = Context::new();

    let data_map = serde_json::to_value(context_data)
        .expect("Failed to serialize context data")
        .as_object()
        .expect("Context data is not a valid object")
        .clone();

    for (key, value) in data_map {
        context.insert(key, &value);
    }

    let rendered = tera
        .render(template_name, &context)
        .map_err(|e| {
            eprintln!("Error on render operation: {:?}", e);
            std::process::exit(1);
        })
        .unwrap();

    Html(rendered)
}
```

<p align="right">(<a href="#top">back to top</a>)</p>

## 👥 Contributing

Contributions are what make the open-source community amazing! Any contributions you make are greatly appreciated.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

## ✉️ Contact

Buğra Okumuş- [@bgraokmush](https://twitter.com/bgraokmush)

<p align="right">(<a href="#top">back to top</a>)</p>

## 🙏 Acknowledgments

Special thanks to Burak Selim Şenyurt for inspiration and guidance!

<p align="right">(<a href="#top">back to top</a>)</p>
