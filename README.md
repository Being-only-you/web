# Being You (BeU)

![BeU Logo](/assets/logo/being-you-colour.svg)

Being You (BeU) is a revolutionary platform that combines uncensored social networking with professional growth opportunities. Our mission is to provide a space where you can express yourself freely and advance your career, all in one place.

## Table of Contents

- [Being You (BeU)](#being-you-beu)
  - [Table of Contents](#table-of-contents)
  - [About the Project](#about-the-project)
    - [Key Principles](#key-principles)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
  - [Development](#development)
  - [Contributing](#contributing)
  - [License](#license)

## About the Project

BeU is designed to be "The social way to work". We're building a free speech and privacy-focused platform that connects people on both social and professional levels. Our goal is to provide you with a tool for professional development while offering an ad-free social networking experience.

### Key Principles

- Freedom of expression (within legal boundaries)
- Privacy protection
- Professional growth
- Ad-free personal profiles

## Features

- **Uncensored Expression**: Express yourself freely without fear of AI-driven censorship.
- **Professional Branding**: Showcase your skills, apply for jobs, and connect with clients.
- **Connecting People**: Build meaningful relationships with friends, family, and professional contacts.
- **Business Tools**: Access job boards, freelancing opportunities, and business management tools.
- **Zero Ads on Personal Profiles**: Enjoy an ad-free experience on your personal profile.
- **Promoted Job Listings**: Find relevant job opportunities on professional profiles.
- **Content Creator Revenue Sharing**: Benefit from your content creation efforts.

## Getting Started

### Prerequisites

- Rust (nightly)
- Node.js and npm
- Cargo and cargo-leptos

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Being-only-you/web.git
   cd being-you
   ```

2. Install dependencies:

   ```bash
   cargo install cargo-leptos
   npm install
   ```

3. Set up environment variables (copy `.env.example` to `.env` and fill in the necessary values)

## Usage

To run the project locally:

```bash
cargo leptos watch
```

This will start the development server. By default, you can access your local project at `http://localhost:3000`.

## Development

BeU is built using the Leptos web framework. The main application logic can be found in `src/app.rs`. Other important directories include:

- `src/components/`: Reusable UI components
- `src/pages/`: Individual page components
- `assets/`: Static assets like images and fonts
- `style/`: CSS and SCSS files

To contribute to the project, please refer to our [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Contributing

We welcome contributions to the BeU project! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) file for details on how to get involved.

## License

This project is released into the public domain under the [Unlicense](LICENSE).

This is free and unencumbered software released into the public domain. Anyone is free to copy, modify, publish, use, compile, sell, or distribute this software, either in source code form or as a compiled binary, for any purpose, commercial or non-commercial, and by any means.

For more information, please refer to <https://unlicense.org>
