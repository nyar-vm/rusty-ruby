# Ruby-WASI

WebAssembly module for Rusty Ruby, enabling Ruby code to run in browser environments.

## 🎯 Project Overview

Ruby-WASI is a WebAssembly module for Rusty Ruby that allows Ruby code to run in browser environments. It provides a lightweight interface for executing Ruby code directly in the browser without the need for a server-side Ruby interpreter.

## 🌟 Key Features

- **Browser Compatibility**: Run Ruby code directly in web browsers
- **Lightweight API**: Simple interface for executing Ruby code
- **Basic Ruby Support**: Supports core Ruby syntax and functionality
- **TypeScript Integration**: Works seamlessly with TypeScript frontend
- **Performance**: Optimized for WebAssembly execution

## 🚀 Quick Start

### Using with TypeScript

```typescript
import { RubyWASI } from './lib/ruby-wasi';

async function runRubyCode() {
  // Initialize Ruby WASI
  const ruby = new RubyWASI();
  
  // Execute Ruby code
  const result = await ruby.evaluate('puts "Hello, WebAssembly!"');
  console.log('Result:', result);
  
  // Define and use Ruby classes
  await ruby.evaluate(`
    class Calculator
      def add(a, b)
        a + b
      end
    end
  `);
  
  const sum = await ruby.evaluate('Calculator.new.add(5, 3)');
  console.log('Sum:', sum); // Output: 8
}

runRubyCode().catch(console.error);
```

### Basic Usage

```typescript
import { RubyWASI } from './lib/ruby-wasi';

const ruby = new RubyWASI();

// Execute simple Ruby expressions
ruby.evaluate('1 + 2 * 3').then(result => {
  console.log('Result:', result); // Output: 7
});

// Run Ruby scripts
ruby.evaluate(`
  def factorial(n)
    return 1 if n <= 1
    n * factorial(n - 1)
  end
  
  factorial(5)
`).then(result => {
  console.log('Factorial of 5:', result); // Output: 120
});
```

## 🏗️ Architecture

### Core Components

- **WebAssembly Module**: Compiled Rusty Ruby runtime for WebAssembly
- **JavaScript/TypeScript Bindings**: Interface for interacting with the WASM module
- **Memory Management**: Efficient memory handling for WebAssembly environment
- **API Layer**: Simple interface for executing Ruby code

## 🛠️ Development

```bash
# Build the WASM module
cargo build --target wasm32-wasi

# Run tests
cargo test

# Optimize the WASM module
wasm-opt -O3 target/wasm32-wasi/debug/ruby-wasi.wasm -o ruby-wasi-optimized.wasm
```

## 📚 Usage Examples

### Interactive Ruby in Browser

```typescript
import { RubyWASI } from './lib/ruby-wasi';

const ruby = new RubyWASI();
const input = document.getElementById('ruby-code');
const output = document.getElementById('output');
const runBtn = document.getElementById('run-btn');

runBtn.addEventListener('click', async () => {
  const code = input.value;
  try {
    const result = await ruby.evaluate(code);
    output.textContent = `Result: ${result}`;
  } catch (error) {
    output.textContent = `Error: ${error.message}`;
  }
});
```

### Ruby in Web Workers

```typescript
// worker.js
import { RubyWASI } from './lib/ruby-wasi';

const ruby = new RubyWASI();

self.addEventListener('message', async (event) => {
  const { code } = event.data;
  try {
    const result = await ruby.evaluate(code);
    self.postMessage({ result });
  } catch (error) {
    self.postMessage({ error: error.message });
  }
});

// main.js
const worker = new Worker('worker.js');

worker.addEventListener('message', (event) => {
  const { result, error } = event.data;
  if (error) {
    console.error('Error:', error);
  } else {
    console.log('Result:', result);
  }
});

worker.postMessage({ code: '1 + 2 * 3' });
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-WASI.
