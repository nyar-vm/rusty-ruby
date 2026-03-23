# Ruby-TS

TypeScript frontend for Rusty Ruby, providing WASM integration and TypeScript bindings for Ruby code execution.

## 🎯 Project Overview

Ruby-TS is a TypeScript package that integrates with Rusty Ruby's WebAssembly module, allowing Ruby code to run in browser environments with type-safe TypeScript bindings.

## 🌟 Key Features

- **WASM Integration**: Seamlessly loads and integrates Rusty Ruby WASM module
- **TypeScript Wrapper**: Provides type-safe TypeScript bindings for Ruby functionality
- **API Export**: Exposes Rusty Ruby core features as a clean TypeScript API
- **Type Safety**: Complete TypeScript type definitions for enhanced development experience

## 🚀 Quick Start

```typescript
import { RBQ } from 'rbq-ts';

async function main() {
  // Initialize RBQ
  const rbq = await RBQ.init();
  
  // Execute Ruby code
  const result = await rbq.evaluate('1 + 2 * 3');
  console.log('Result:', result);
  
  // Define and use Ruby classes
  await rbq.evaluate(`
    class Person
      def initialize(name)
        @name = name
      end
      
      def greet
        "Hello, #{@name}!"
      end
    end
  `);
  
  const person = await rbq.evaluate('Person.new("World")');
  const greeting = await rbq.evaluate('person.greet');
  console.log('Greeting:', greeting);
}

main().catch(console.error);
```

## 🏗️ Architecture

- **WASM Loader**: Responsible for loading and initializing the Rusty Ruby WASM module
- **API Wrapper**: Provides a clean TypeScript API around the WASM functionality
- **Type Definitions**: Complete TypeScript types for all exposed Ruby features
- **Error Handling**: Comprehensive error handling for Ruby execution

## 🛠️ Development

```bash
# Build the project
pnpm build

# Development mode
pnpm dev
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-TS.
