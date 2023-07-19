# YaDI Design

## Table of contents

1. [Glossary](#glossary)
2. [Functional requirements](#functional-requirements)
3. [Non-functional requirements](#non-functional-requirements)

## Glossary
1. **Container** is a read-only collection of **entries**.
2. **Entry** is a descriptor of some struct and its **dependencies**.
3. **Dependency** is an **entry** that needs to be instantiated to resolve the current **entry**
4. **Lifetime** is a property of an **entry** that describes the **entry** life scope.
   1. **Singleton** - the **entry** lives during the whole app running process.
   2. **Scoped** - TBD
   3. **Transient** - the **entry** instantiated every time it is requested from a **container**
5. **Anchor** TBD

## Functional requirements
1. YaDI has to provide tools to collect user traits, structs, enums
2. YaDI has to resolve dependencies of user traits, structs, enums

## Non-functional requirements
1. TBD