# Flyoff

Flyoff is a project that uses Rust and WebAssembly (WASM) to train virtual birds to get the most food using a genetic algorithm and a custom neural network.



https://user-images.githubusercontent.com/79325116/213137988-a7e9a21d-5fdf-423a-a583-b205510a9291.mp4



## Prerequisites

- Rust
- wasm-pack
- Node.js and npm

## Installation

1. Clone the repository: `git clone https://github.com/ipriyam26/Flyoff.git`
2. Navigate to the project directory: `cd Flyoff`
3. Build the project: `wasm-pack build`
4. Link the project: `wasm-pack link`
5. Install the npm dependencies: `npm install`

## Usage

1. Start the development server: `npm run start`
2. Open `http://localhost:8080/` in your browser to view the simulation.
3. The birds will start to gather food using the Neural Network and Genetic Algorithm

## Neural Network

The project uses a custom neural network architecture, which can be found in the `src/nn` directory. The network takes in the bird's current x and y position, as well as the positions of nearby food, and outputs a prediction for the bird's next move.

## Genetic Algorithm

The project uses a genetic algorithm to train the neural network. Each bird has a set of weights for its neural network, and these weights are evolved over time by selecting the birds with the highest food gathering efficiency to breed and pass on their weights to the next generation.

## Customization

You can change the parameters of the simulation and the Neural Network by modifying the `config.js` file

## Contributing

If you're interested in contributing to Flyoff, please feel free to open a pull request or an issue.

## License

Flyoff is licensed under the MIT license. See [LICENSE](https://github.com/ipriyam26/Flyoff/blob/master/LICENSE) for more information.
