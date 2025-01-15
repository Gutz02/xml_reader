XML Reader is a Rust-based application designed to parse and process XML files efficiently. It constructs a hierarchical tree structure from XML data, enabling easy traversal and manipulation of nodes and their attributes.

Features
XML Parsing: Converts XML content into a tree of nodes, preserving parent-child relationships.
Attribute Extraction: Retrieves attributes associated with XML tags.
Text Content Handling: Processes and stores text within XML elements.
Self-Closing Tag Support: Recognizes and correctly handles self-closing XML tags.
Rust Concepts Utilized
This project showcases several advanced Rust programming concepts:

Ownership and Borrowing
The application employs Rust's ownership model to manage memory safely:

Parent-Child Relationships: Parent nodes maintain ownership of their child nodes using Rc<Node>, allowing multiple references.
Weak References: Child nodes hold weak references (Weak<Node>) to their parents to prevent reference cycles, ensuring proper memory management.
Smart Pointers
The project makes extensive use of Rust's smart pointers:

Rc<T>: Used for reference-counted ownership, allowing multiple parts of the code to own a node.
Weak<T>: Employed to create non-owning references to prevent circular dependencies.
RefCell<T>: Allows for interior mutability, enabling mutation of data even when there are immutable references.
Pattern Matching
Rust's powerful pattern matching is utilized to parse and process XML lines efficiently:

Tag Detection: Determines whether a line represents an opening tag, closing tag, or self-closing tag using pattern matching.
Attribute Parsing: Extracts key-value pairs from tag attributes through pattern matching techniques.
Error Handling
The application incorporates robust error handling mechanisms:

Option and Result Types: Utilizes Option and Result types to handle potential errors gracefully during parsing and node manipulation.
Unwrap and Expect: Employs .unwrap() and .expect() judiciously, ensuring that any unexpected None or Err values are caught during development.
Code Structure
main.rs: The entry point of the application, handling user input and orchestrating the parsing process.
node.rs: Defines the Node struct and implements methods for node manipulation, including setting parents and children, and retrieving attributes.
parser.rs: Contains functions responsible for reading XML lines, detecting node types, and constructing the node tree.
