# rusty_logic
Logic Gate based Simulations Engine written in Rust

## Idea
Building a modular and extendible node based cli simulation engine.


## Key Features
<span style="color:red">
under development
</span>
<span style="color:green">
developed
</span>

+ <span style="color:green"> CLI </span> for quick node selection and network setup
+ Different Node Types
    + <span style="color:green"> Logic/Discrete Nodes </span> (on/off Nodes with Logic operation)
    + <span style="color:red"> Continous/dynamic Nodes </span> (Nodes with Bias and Decay -> Integration of environmental signals)
            + Biased nodes (Bias can change based on Environmental signals)
            + implented decay function impacting $`I(t)_{Bias} = I(0)_{Bias}e^{-\tau*t}`$
    + <span style="color:red"> Diffusive Nodes </span> (Continous Node with diffusion upon activation )
            + Biased nodes (Bias can change based on Environmental signals)
            + Implented decay function given by $`I(t)_{Bias} = I(0)_{Bias}e^{-\tau*t}`$
            + Diffusion to $n$ neighbors upon activation
    + <span style="color:red"> Stochastic Nodes </span> (Stochastic Noise gates)
            + Input gates that displaces gate bias stoachstically

## Use cases 
+ Lightweight Testing Framework for Mechanical Modelling
+ Multi Network Modelling

## Final testing
Modelling of Action Potentials and testing agains Hodging Huxley results. 