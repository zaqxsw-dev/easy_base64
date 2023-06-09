<a name="readme-top"></a>



<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">easy_base64</h3>

  <p align="center">
    Simple, small and fast, dependency free base64 encoder/decoder.
    <br />
    <a href="https://github.com/zaqxsw-dev/easy_base64"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/zaqxsw-dev/easy_base64/issues">Report Bug</a>
    ·
    <a href="https://github.com/zaqxsw-dev/easy_base64/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

I have created a library for people who simply need to encode and decode data to and from Base64 with a straightforward API. The purpose of this library is to reduce the time between when a person discovers the library and when they start using it. Additionally, the aim is to avoid burdening the project with extra dependencies and increasing compilation time.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

Just add new dependencie in your Cargo.toml file.

* Cargo.toml
  ```toml
  [dependencies]
  easy_base64 = "0.1"
  ```

<!-- USAGE EXAMPLES -->
## Usage

```rust
use easy_base64::{decode, encode};

fn main() {
    let decoded_b64 = decode(b"U3U=");
    println!("Decoded: {}", String::from_utf8(decoded_b64).unwrap());

    let encoded_b64 = encode(b"Su");
    println!("Encoded: {}", encoded_b64);

    // if you have String
    let decoded_b64 = decode(String::from("U3U=").as_bytes());
    println!("Decoded: {}", String::from_utf8(decoded_b64).unwrap());

    let encoded_b64 = encode(String::from("Su").as_bytes());
    println!("Encoded: {}", encoded_b64);
}
```

_Why encode return a String, but decode return a bytes?_

Because base64 encoded the result always contains valid printable string symbols. But if was encoded binary data, then decode result not will be valid string.

_For more examples, please refer to the [Documentation](https://docs.rs/easy_base64/0.1.0/easy_base64/)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Bogdan Lipovtsev - megafreelancer2012@gmail.com

Project Link: [https://github.com/zaqxsw-dev/easy_base64](https://github.com/zaqxsw-dev/easy_base64)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/zaqxsw-dev/easy_base64.svg?style=for-the-badge
[contributors-url]: https://github.com/zaqxsw-dev/easy_base64/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/zaqxsw-dev/easy_base64.svg?style=for-the-badge
[forks-url]: https://github.com/zaqxsw-dev/easy_base64/network/members
[stars-shield]: https://img.shields.io/github/stars/zaqxsw-dev/easy_base64.svg?style=for-the-badge
[stars-url]: https://github.com/zaqxsw-dev/easy_base64/stargazers
[issues-shield]: https://img.shields.io/github/issues/zaqxsw-dev/easy_base64.svg?style=for-the-badge
[issues-url]: https://github.com/zaqxsw-dev/easy_base64/issues
[license-shield]: https://img.shields.io/github/license/zaqxsw-dev/easy_base64.svg?style=for-the-badge
[license-url]: https://github.com/zaqxsw-dev/easy_base64/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/bogdan-lipovtsev-746946257
