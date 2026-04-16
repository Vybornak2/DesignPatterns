# Behavioral Patterns Overview

Behavioral design patterns are concerned with algorithms and the assignment of responsibilities between objects. They describe not just patterns of objects or classes but also the patterns of communication between them.

## Chain of Responsibility

- __Popularity: ★★★☆☆__
- __Complexity: ★★★☆☆__

### Intent:
__Chain of Responsibility__ is a behavioral design pattern that passes requests along a chain of handlers. Upon receiving a request, each handler decides either to process the request or to pass it to the next handler in the chain.

[Chain of Responsibility details](./chain_of_responsibility.ipynb)

---

## Command

- __Popularity: ★★★★☆__
- __Complexity: ★★☆☆☆__

### Intent:
__Command__ is a behavioral design pattern that turns a request into a stand-alone object containing all information about the request. This transformation allows you to parameterize methods with different requests, delay or queue a request's execution, and support undoable operations.

[Command details](./command.ipynb)

---

## Observer

- __Popularity: ★★★★★__
- __Complexity: ★★☆☆☆__

### Intent:
__Observer__ is a behavioral design pattern that defines a one-to-many dependency between objects so that when one object changes state, all its dependents are notified and updated automatically.

[Observer details](./observer.ipynb)

---

## State

- __Popularity: ★★★☆☆__
- __Complexity: ★★★☆☆__

### Intent:
__State__ is a behavioral design pattern that lets an object alter its behavior when its internal state changes. It appears as if the object changed its class.

[State details](./state.ipynb)

---

## Interpreter

- __Popularity: ★★☆☆☆__
- __Complexity: ★★★★☆__

### Intent:
__Interpreter__ is a behavioral design pattern that defines a representation for a language's grammar along with an interpreter that uses the representation to interpret sentences in the language.

[Interpreter details](./interpreter.ipynb)

---

## Strategy

- __Popularity: ★★★★☆__
- __Complexity: ★★☆☆☆__

### Intent:
__Strategy__ is a behavioral design pattern that defines a family of algorithms, encapsulates each one, and makes them interchangeable. Strategy lets the algorithm vary independently from the clients that use it.

[Strategy details](./strategy.ipynb)

---

## Memento

- __Popularity: ★★☆☆☆__
- __Complexity: ★★★☆☆__

### Intent:
__Memento__ is a behavioral design pattern that lets you save and restore the previous state of an object without revealing the details of its implementation.

[Memento details](./memento.ipynb)

---

## Iterator

- __Popularity: ★★★★★__
- __Complexity: ★★☆☆☆__

### Intent:
__Iterator__ is a behavioral design pattern that lets you traverse elements of a collection without exposing its underlying representation (list, stack, tree, etc.).

[Iterator details](./iterator.ipynb)

---

## Template Method

- __Popularity: ★★★☆☆__
- __Complexity: ★★☆☆☆__

### Intent:
__Template Method__ is a behavioral design pattern that defines the skeleton of an algorithm in the superclass but lets subclasses override specific steps of the algorithm without changing its structure.

[Template Method details](./template_method.ipynb)

---
