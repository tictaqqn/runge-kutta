# Simulation on Birds Swarms

Solving ODE with Runge-Kutta method and automatic differentiation.

## Models

Assume N birds flying in swarms. Let x be positon of i-th birds, and v0 be preferred speed. Newton's equation of each bird is

<img src="https://latex.codecogs.com/gif.latex?\dot&space;v_i&space;=&space;\gamma&space;(v_0^2&space;-&space;{\dot&space;x_i}^2)&space;\dot&space;x_i&space;&plus;&space;\sum_{j&space;\neq&space;i&space;}&space;f(x_j&space;-&space;x_i)&space;&plus;&space;\sum_{j&space;\neq&space;i&space;}&space;g(x_j&space;-&space;x_i,&space;v_j&space;-&space;v_i)" title="\dot v_i = \gamma (v_0^2 - {\dot x_i}^2) \dot x_i + \sum_{j \neq i } f(x_j - x_i) + \sum_{j \neq i } g(x_j - x_i, v_j - v_i)" />

where

<img src="https://latex.codecogs.com/gif.latex?f(x)&space;=&space;-&space;\nabla&space;V(x)" title="f(x) = - \nabla V(x)" />

<img src="https://latex.codecogs.com/gif.latex?V(x)&space;=&space;A_1&space;\exp&space;(-(x/R_1)^2)&space;-&space;A_2&space;\exp&space;(-(x/R_2)^2)" title="V(x) = A_1 \exp (-(x/R_1)^2) - A_2 \exp (-(x/R_2)^2)" />

<img src="https://latex.codecogs.com/gif.latex?g(x,&space;v)&space;=&space;A_v&space;\exp&space;(-(x/&space;R_v)^2)&space;v" title="g(x, v) = A_v \exp (-(x/ R_v)^2) v" />

V is Mexican-hat potential and f expresses preferred distance between birds, whereas g aligns their velocity with that of neighborhood.

## Results

![Results](https://user-images.githubusercontent.com/38175513/79634941-a327ba00-81a8-11ea-8c83-77d4dd622dae.gif)

## Reference

Gros C.(2015). *Complex and adaptive dynamical systems* , Springer
