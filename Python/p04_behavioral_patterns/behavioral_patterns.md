# Behavioral Patterns Overview

Behavioral design patterns are concerned with algorithms and the assignment of responsibilities between objects.

- [Chain of Responsibility](./chain_of_responsibility.ipynb) passes requests along a chain of handlers, where each handler decides to process the request or pass it to the next handler.

- [Command](./command.ipynb) turns a request into a stand-alone object, allowing you to parameterize methods, queue or delay execution, and support undoable operations.

- [Observer](./observer.ipynb) defines a one-to-many dependency between objects so that when one object changes state, all its dependents are notified and updated automatically.

- [State](./state.ipynb) lets an object alter its behavior when its internal state changes, appearing as if the object changed its class.

- [Interpreter](./interpreter.ipynb) defines a representation for a language's grammar along with an interpreter that uses it to interpret sentences in the language.

- [Strategy](./strategy.ipynb) defines a family of algorithms, encapsulates each one, and makes them interchangeable so the algorithm can vary independently from the clients that use it.

- [Memento](./memento.ipynb) lets you save and restore the previous state of an object without revealing the details of its implementation.

- [Iterator](./iterator.ipynb) lets you traverse elements of a collection without exposing its underlying representation.

- [Template Method](./template_method.ipynb) defines the skeleton of an algorithm in the superclass but lets subclasses override specific steps without changing its structure.
