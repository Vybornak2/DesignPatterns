# Structural Patterns Overview

Structural design patterns explain how to assemble objects and classes into larger structures while keeping these structures flexible and efficient.

## Adapter

- __Popularity: ★★★★☆__
- __Complexity: ★★☆☆☆__

### Intent:
__Adapter__ is a structural design pattern that allows objects with incompatible interfaces to collaborate. It acts as a bridge between two incompatible interfaces, making them work together without modifying their code.

[Adapter details](./adapter.ipynb)

---

## Decorator

- __Popularity: ★★★★★__
- __Complexity: ★★★☆☆__

### Intent:
__Decorator__ is a structural design pattern that attaches additional responsibilities to objects dynamically. Decorators provide a flexible alternative to subclassing for extending functionality.

[Decorator details](./decorator.ipynb)

---

## Bridge

- __Popularity: ★★★☆☆__
- __Complexity: ★★★★☆__

### Intent:
__Bridge__ is a structural design pattern that separates an abstraction from its implementation so that the two can vary independently. This pattern involves an interface which acts as a bridge between the abstract class and implementer classes.

[Bridge details](./bridge.ipynb)

---

## Facade

- __Popularity: ★★★★☆__
- __Complexity: ★☆☆☆☆__

### Intent:
__Facade__ is a structural design pattern that provides a simplified interface to a complex subsystem. It defines a higher-level interface that makes the subsystem easier to use.

[Facade details](./facade.ipynb)

---

## Flyweight

- __Popularity: ★★☆☆☆__
- __Complexity: ★★★★☆__

### Intent:
__Flyweight__ is a structural design pattern that minimizes memory usage by sharing common parts of state between multiple objects. It's used when you need to create a large number of similar objects efficiently.

[Flyweight details](./flyweight.ipynb)

---

## Proxy

- __Popularity: ★★★☆☆__
- __Complexity: ★★★☆☆__

### Intent:
__Proxy__ is a structural design pattern that provides a surrogate for another object to control access to it. It creates a representative object that controls access to another object, which may be remote, expensive to create, or in need of security.

[Proxy details](./proxy.ipynb)

---

## Composite

- __Popularity: ★★★★☆__
- __Complexity: ★★★☆☆__

### Intent:
__Composite__ is a structural design pattern that composes objects into tree structures to represent hierarchies. It lets clients treat individual objects and compositions of objects uniformly.

[Composite details](./composite.ipynb)

---

## Pattern Comparisons

### Adapter vs Bridge:
- **Adapter** adds compatibility between classes that have different interfaces.
- **Bridge** decouples an abstraction from its implementation so that the two can vary independently.

### Decorator vs Proxy:
- **Decorator** adds responsibilities to objects dynamically.
- **Proxy** controls access to objects.
