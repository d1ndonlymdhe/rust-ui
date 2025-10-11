# Rust UI Framework with Chat Application

A custom UI framework built in Rust using Raylib, featuring a complete chat application as a demonstration. This project showcases modern Rust patterns including thread-safe state management, component-based architecture, and deadlock-free concurrent programming.

## 🚀 Features

### UI Framework
- **Component-Based Architecture**: Modular, reusable UI components
- **Flexible Layout System**: Row/column layouts with flexbox-like properties
- **Rich Text Support**: Text rendering with customizable fonts, colors, and sizing
- **Interactive Elements**: Buttons, text inputs with click and keyboard handlers
- **Thread-Safe State Management**: Global state using `Arc<Mutex<T>>` with lazy static
- **Event System**: Mouse and keyboard event handling with propagation control

### Chat Application
- **Real-time UI Updates**: Dynamic rendering of messages and user interactions
- **User Management**: Multiple users with visual indicators for active conversations
- **Message Threading**: Separate conversations between different users
- **Text Input**: Real-time text editing with keyboard support
- **Visual Feedback**: Color-coded messages and interactive user selection

## 🏗️ Architecture

### Core Components

#### UI Framework (`src/ui/`)
```
src/ui/
├── common.rs      # Base traits, enums, and utilities
├── layout.rs      # Flexible layout container (Row/Column)
├── text_layout.rs # Text rendering component
├── text_input.rs  # Interactive text input component
├── raw_text.rs    # Basic text rendering
└── root.rs        # Root UI container and event manager
```

#### Application (`src/main.rs`)
- **Global State**: Thread-safe chat state using `lazy_static!` and `Arc<Mutex<T>>`
- **Component Functions**: Modular functions for different UI sections
- **Event Loop**: Raylib-based rendering and input handling

### State Management

The application uses a global state pattern with proper concurrency control:

```rust
lazy_static! {
    static ref CHAT_STATE: Arc<Mutex<ChatState>> = Arc::new(Mutex::new(ChatState::new()));
}
```

**Key Features:**
- **Deadlock Prevention**: All data extraction happens in single lock acquisitions
- **Thread Safety**: `Arc<Mutex<T>>` ensures safe concurrent access
- **Clean Separation**: UI components and state management are decoupled

### Component Architecture

#### Base Trait System
All UI components implement the `Base` trait providing:
- **Rendering**: `draw()` method for visual output
- **Layout**: Two-pass layout system (`pass_1`, `pass_2`)
- **Events**: Mouse and keyboard event handling
- **Hierarchy**: Parent-child relationships and tree traversal

#### Layout System
- **Flexible Dimensions**: `FILL`, `FIT`, `FIXED(px)`, `PERCENT(%)`
- **Flex Properties**: CSS-like flex system for responsive layouts
- **Alignment**: Start, Center, End alignment for both axes
- **Padding & Gaps**: Spacing control for polished layouts

## 🛠️ Dependencies

```toml
[dependencies]
lazy_static = "1.5.0"    # Global static state management
raylib = "5.5.0"         # Graphics and windowing
uuid = "1.18.1"          # Unique ID generation
```

## 🚦 Getting Started

### Prerequisites
- Rust 2024 edition or later
- Raylib system dependencies (varies by platform)

### Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd rust-ui

# Build the project
cargo build

# Run the chat application
cargo run
```

### Controls
- **Mouse**: Click on users to switch conversations
- **Keyboard**: Type in the text input field
- **Send**: Click the "Send" button to send messages
- **Backspace**: Delete characters from the input

## 📁 Project Structure

```
rust-ui/
├── src/
│   ├── main.rs              # Application entry point and chat logic
│   └── ui/                  # UI framework modules
│       ├── common.rs        # Core traits and utilities
│       ├── layout.rs        # Layout container component
│       ├── text_layout.rs   # Text display component
│       ├── text_input.rs    # Interactive text input
│       ├── raw_text.rs      # Basic text rendering
│       └── root.rs          # Root container and event handling
├── Cargo.toml              # Project dependencies
└── README.md              # This file
```

## 🎯 Key Components

### Chat Application Components

1. **`users_header()`**: Header section for the users list
2. **`users_component()`**: Interactive list of available users
3. **`message_component()`**: Individual message bubble rendering
4. **`input_box_component()`**: Text input with real-time editing
5. **`send_button_component()`**: Send button with click handling
6. **`messages_component()`**: List of messages for current conversation
7. **`input_row_component()`**: Combined input box and send button
8. **`left_sidebar_component()`**: Complete left sidebar with users
9. **`chat_area_component()`**: Main chat area with messages and input
10. **`chat_layout()`**: Root layout combining sidebar and chat area

### UI Framework Components

1. **`Layout`**: Flexible container with row/column layout
2. **`TextLayout`**: Rich text display with alignment and styling
3. **`TextInput`**: Interactive text input with keyboard handling
4. **`RawText`**: Basic text rendering for simple cases
5. **`Root`**: Top-level container managing focus and events

## 🔒 Concurrency & Safety

### Deadlock Prevention
- **Single Lock Pattern**: Extract all needed data in one lock acquisition
- **Early Release**: Release locks before entering iterators or closures
- **Clone Strategy**: Clone data instead of holding references across lock boundaries

### Thread Safety
- **Arc<Mutex<T>>**: Thread-safe reference counting with mutual exclusion
- **Interior Mutability**: `Rc<RefCell<T>>` for single-threaded shared ownership
- **Event Handlers**: Closures safely capture and modify state

## 🎨 Styling and Layout

### Color Scheme
- **Background**: Blue chat area, red sidebar
- **Messages**: Green for user messages, slate blue for others
- **UI Elements**: Light gray inputs, dark gray buttons
- **Selected User**: Light green highlight

### Layout Properties
- **Responsive**: Flex-based sizing (2.5:7.5 ratio for sidebar:chat)
- **Padding**: Consistent spacing throughout the interface
- **Typography**: Clear font sizes (20px messages, 24px headers)

## 🧪 Development Features

### Debugging Support
- **Debug Names**: Each component has a unique identifier
- **Event Tracing**: Mouse and keyboard event propagation tracking
- **Layout Inspection**: Two-pass layout system for debugging

### Extensibility
- **Modular Design**: Easy to add new component types
- **Event System**: Flexible event handling and propagation
- **Theme Support**: Color and styling can be easily customized

## 🚀 Future Enhancements

### Planned Features
- **Scrollable Containers**: Handle overflow content