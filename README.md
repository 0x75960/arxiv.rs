# arxiv.rs

arxiv api client 

* **(Currently working in progress)** I may make major changes soon.

```toml
[dependencies]
arxiv = {version = "*", git="https://github.com/0x75960/arxiv.rs"}
```

## example

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    arxiv::search("all:electron&start=0&max_results=10")?
        .iter()
        .enumerate()
        .for_each(|(idx, x)| {println!("{}: {}", idx+1, x.title)});
    Ok(())
}
```

got:

```console

1: Impact of Electron-Electron Cusp on Configuration Interaction Energies
2: Electron thermal conductivity owing to collisions between degenerate
  electrons
3: Electron pairing: from metastable electron pair to bipolaron
4: Hamiltonian of a many-electron system with single-electron and
  electron-pair states in a two-dimensional periodic potential
5: Electron-Electron Bremsstrahlung Emission and the Inference of Electron
  Flux Spectra in Solar Flares
6: Improved scenario of baryogenesis
7: Exact Electron-Pairing Ground States of Tight-Binding Models with Local
  Attractive Interactions
8: Electron-electron interactions in a weakly screened two-dimensional
  electron system
9: Free-electron properties of metals under ultrafast laser-induced
  electron-phonon nonequilibrium: A first-principles study
10: First-principles calculations of hot-electron lifetimes in metals
```
