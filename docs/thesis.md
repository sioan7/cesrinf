# 1. Introduction

*The introduction of the thesis has three main parts divided in the 2 subchapters below:*

1. *Context and motivation – What is the domain where the work is conducted. What is the problem identified? How can the problem be measured? What metric is used? (e.g. energy consumption, request/second, throughput, runtime, frames/second, accuracy, etc.). How does the current thesis address these issues? Give a short description of the solution, and how it will tackle the problems identified.* 
2. *The thesis structure.*

## 1.1. Project Context

> **TODO**


Contrast WASM uses originally in the browser and later uses on the server
Compare and contrast WASM runtimes with JVM-based VMs
## 1.2. Thesis Structure

> **TODO**
# 2. Project Objectives and Requirements

*This section presents the paper objectives, functional and non-functional requirements and metrics used to evaluate the solutions.*

## 2.1. Main Objective

*The main objective of this thesis is to study, design and implement an…*

>Take 1:
The main objective of this thesis is to study the differences between several distribution types for typical Web servers by designing and implementing the said server and a suite of tests.

>Take 2:
The main objective of this thesis is to:
- design and implement web servers with (with the same features) for 3 different compilation targets: Linux, WASM (wasmtime and Wasmer)
- package the servers to the corresponding containers: Docker, OCI-compliant container
- design and implement a load test that stresses one API endpoint (this endpoint takes as input a CESR-encoded stream and returns a JSON array containing the messages in the stream)
- compare and contrast the containers at build and run time with regards to agreed-upon metrics

## 2.2. Secondary Objectives

| **No.** | **Objective**                                | **Description**                                                                                                              | **Reference** |
| ------- | -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ------------- |
| 0       | Study application-level virtualization types | Provide a brief description of already existing comparative analyses.                                                        | Ch. 3         |
| 1       | Study OS-level virtualization types.         | Provide a brief description of the most common OS-level virtualization types (CONTAINER)                                     | Ch. 3         |
| 5       | WASM distribution via OCI containers         | Provide a brief description of how WASM binaries and any additional resources can be distributed as OCI-compliant containers | Ch. 3         |
|         |                                              |                                                                                                                              |               |
|         |                                              |                                                                                                                              |               |
|         |                                              |                                                                                                                              |               |
## 2.3. Requirements
### 2.3.1. Functional Requirements
## 2.3.2. Non-Functional Requirements

## 2.4. Metrics used for evaluation

*Formulas and explanations for metrics given in introduction to evaluate solution.* 

Metrics:
- image/binary size (asta face diferenta mare, wasm binaries au dimensiuni cu pana la 2 ordine de marime mai mici decat Docker images)
- startup time
- runtime overhead (asta nu e foarte concret, mai trebuie sa investighez)
- memory usage
- runs in the browser
- portability
- standards
- system interaction
- load test si parsarea mi se pare un use case bun, parsarea, de obicei de JSON, are overhead mare la runtime, mai ales pentru high-throughput applications

See: https://medium.com/@shivraj.jadhav82/webassembly-wasm-docker-vs-wasm-275e317324a1
# 3. Bibliographic research

*This section should contain about 20-30 papers studied in the domain. The papers should be classified in 2 or 3 directions. In the end, it should be highlighted what the proposed solution brings new compared to the state of the art.*

*E.g.*

*The domain of optimizing the DC energy consumption is vast and there are various approaches. The state-of-the art solutions are classified in two directions: consolidation approaches that take into account only server power consumption and energy optimization techniques that take into account both the server and the cooling system of the DC*

1. *Consolidation Methods in Data Centers*

*One of the reasons for high energy consumption in DCs is the inefficient resource usage. For instance, servers usually operate at 10% up to 50% of their maximum capacity*

## 3.1 Direction 1
## 3.2 Direction 2
## 3.3 Direction 3
# 4. Solution – part 1

The solution should be presented in 1-2 chapters. If the solution is both a theoretical one and a practical one (the actual implementation), then 2 chapters should be used. In these sections, you should present: 

- Formulas and mathematical models
- Data structures
- Algorithms
- System Conceptual Architecture
- System Deployment architecture
# 5. Solution – part 2

# 6. Experiments and Validation

The experimental section should present an evaluation of the proposed solution, considering a set of metrics. The section is split in two parts:

1. Experimental Setup

This section should present:

- Data sources for data used in experiments (real or simulated)
- Metrics used to evaluate the solution
- Experimental setup description
- Algorithms and parameter values used in experiments
- Classical Algorithms/Methods used in comparisons. 

1. Experimental Results

This section should contain the scenarios used in experiments. The section can be either be constructed around a series of scenarios, or an exhaustive evaluation of the metrics on a large dataset can be shown. 

# 7. User Manual

1. Installation Prerequisites

- Hardware and software requirements

1. User Interaction

- Print screens of the application explained

# 8. Conclusion 

The conclusion of the thesis should close the objectives proposed in the second section. Furthermore, relevant figures of the evaluation section should be given to show the quality of the proposed solution

# 9. References

## 9.2 Papers

Next-generation Web Applications with WebAssembly and TruffleWasm
https://www.researchgate.net/publication/357069460_Next-generation_Web_Applications_with_WebAssembly_and_TruffleWasm

Bringing the Web up to Speed with WebAssembly
https://people.mpi-sws.org/~rossberg/papers/Haas,%20Rossberg,%20Schuff,%20Titzer,%20Gohman,%20Wagner,%20Zakai,%20Bastien,%20Holman%20-%20Bringing%20the%20Web%20up%20to%20Speed%20with%20WebAssembly.pdf

Understanding the Performance of WebAssembly Applications
https://weihang-wang.github.io/papers/imc21.pdf

An Empirical Study of Real-World WebAssembly Binaries: Security, Languages, Use Cases
https://dlehmann.eu/publications/WasmBench-www2021.pdf

Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code
https://www.usenix.org/system/files/atc19-jangda.pdf

Provably-Safe Multilingual Software Sandboxing using WebAssembly
https://www.usenix.org/system/files/sec22-bosamiya.pdf

Wasabi: A Framework for Dynamically Analyzing
WebAssembly
https://software-lab.org/publications/asplos2019_Wasabi.pdf

Empowering Web Applications with WebAssembly:
Are We There Yet?
https://weihang-wang.github.io/papers/wasm-ase21nier.pdf

Everything Old is New Again:
Binary Security of WebAssembly
https://www.usenix.org/system/files/sec20-lehmann.pdf

## 9.2 Articles

https://www.fermyon.com/blog/webassembly-vs-containers

https://www.fermyon.com/blog/webassembly-wasi-and-the-component-model

---

https://github.com/mbasso/awesome-wasm?tab=readme-ov-file#papers
