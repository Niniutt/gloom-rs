
## TDT4195: Visual Computing Fundamentals
# Computer Graphics : assignment 1

## Task 1: Drawing your first triangle [2.5 points]

### c)

Following the setup phase, we follow the a) and b) questions until reaching this point.

![
    5 triangles built
](images/5triangles.png)

The coordinates are the following :

```rust
let vertices: Vec<f32> = vec![-0.6, -0.6, 0.0, 0.6, -0.6, 0.0, 0.0, 0.6, 0.0,
                              -0.8, 0.2, 0.0, -0.7, 0.2, 0.0, -0.65, 0.4, 0.0,
                              -0.6, -0.3, 0.0, 0.0, 0.8, 0.0, -0.6, 0.6, 0.0,
                              1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                              0.0, -0.7, 0.0, 0.4, -0.9, 0.0, 0.8, -0.8, 0.0];
let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
```

We can go on to the next task.

## Task 2: Geometry and Theory [1.5 point]

### a) Not a triangle

![
    Not a triangle
](images/notatriangle.png)

**i) What is the name of this phenomenon?**

The name of this phenomenon is clipping. 
The parts of the triangle which get out of the clipbox, a cube of dimension 2x2x2 ranging between the values of -1 and 1 on each axis, are cut out of the rendering process.

**ii) When does it occur?**

It occurs in the Vertex Post-Processing step of the OpenGL pipeline, after the Vertex Shader has been applied.

**iii) What is its purpose?**

It's purpose is to avoid making unnecessary operations on parts of fragments outside of the frame, so as to reduce the cost of the total operations, thus increasing the rendering rate of the pipeline.

### b) Modifying the index buffer

From the triangle used before, I set every `z` coordinate to `0.0` and I exchanged the order of the indices array from `[0, 1, 2]` to :

```rust
let vertices: Vec<f32> = vec![0.6, -0.8, 0.0, 0.0, 0.4, 0.0, -0.8, -0.2, 0.0];
let indices: Vec<u32> = vec![0, 2, 1];
```

**i) What happens?**

The triangle disappears !

![
    No triangle anymore
](images/triangledisappeared.png)

**ii) Why does it happen?**

While switching two elements' places into the indices array, we made the vertices order of drawing in the canvas go from anticlockwise to clockwise.
However, this order is used to determine if the primitive shape is facing towards the spectator or the back.
If it is oriented towards the back, the primitive won't be rendered. 
This is another way of decreasing the total cost of the operations : avoiding calculus on fragments that will not be seen.

**iii) What is the condition under which this effect occurs? Determine a rule.**

The condition is thus if the order of the vertices make the triangle be drawn anticlockwise, the primitive shape is facing the spectator (the eye), and will be rendered. 
This way, if we move around such as the back becomes front and front becomes back, clockwise becomes anticlockwise, anticlockwise becomes clockwise and the shapes now facing the back are not rendered. 
It is a cunning logic.

### c) Explain the following in your own words

**i) Why does the depth buffer need to be reset each frame?**

Not resetting the depth buffer would mean some information from the last frame that are not up to date anymore could be kept through the next frame.
The depth buffer needs to be reset each frame to prevent artifacts in case an object moved (which is a condition you can't monitor precisely and efficiently for each frame).

**ii) In which situation can the Fragment Shader be executed multiple times for the
same pixel? (Assume we do not use multisampling.)**

In case multiple primitive shapes overlap on the same pixel, the fragment will be used multiple times for the same pixel. 
For example, the alpha of the fragment being calculated may imply that the final color will depend on previous fragment shader calculation on the same pixel.

**iii) What are the two most commonly used types of Shaders?
What are the responsibilities of each of them?**

The two most commonly used types of shaders are :

* Vertex shaders
* Fragment shaders

The **Vertex Shader** is responsible for applying a transformation on each vertex present in the scene, and takes parameters from the input buffers.
The transformation can impact each parameter of a vertex, which may be spatial coordinates, UV coordinates, color, alpha, custom parameters, ... .
Thus the actions performed range from spatial transformations to calculation for color rendering / light management.

The **Fragment Shader** is different from a "pixel shader", as explained in the last question.
Its operations take place after the Fragment Generation stage of the rendering pipeline.
The fragment shader will apply a color to a pixel, and may do so multiple times. 
The calculation involved to obtain this color will depend on color, lighting, depth, alpha, and other custom additions.

**iv) Why is it common to use an index buffer to specify which vertices should be
connected into triangles, as opposed to relying on the order in which the vertices
are specified in the vertex buffer(s)?**

Relying only on the order in which the vertices are specified in the vertex buffer(s) would imply some repetitions.
Complex shapes in 3D are made from small triangles (or maybe other primitive shapes) linked together in that they share one or two (or three if there is texture on both sides) vertices.
In that case, the shared vertices would be present in multiple places in the vertex buffer(s). 

That would not be that much of a problem if the data was just an unsigned integer. However the data for each vertices is often a number of float parameters, which takes up a lot of space considering the enormous amount of vertices there usually is.

Which is why the index buffer takes this "repetition" responsibility using the smallest memory numbers. That way each vertex is written once and memory space is saved.

**v) While the last input of gl::VertexAttribPointer() is a pointer, we usually pass
in a null pointer. Describe a situation in which you would pass a non-zero value
into this function.**

> "The pointer parameter defines the number of bytes until the first value in the
buffer"

*Source : Assignment 1 tutorial*

A situation in which you would pass a non-zero value into this function is therefore a situation where you cannot calculate this number of bytes from the other parameters.
Adding for example other information in the vertex, like UV coordinates, and an alpha coordinate. 
In this situation, the pointer would have to represent the total size of all these coordinates.

### d) Modify the shader pair

**i) Mirror/flip the whole scene both horizontally and vertically at the same time**

This is the basis triangle :

![
    Just a normal triangle
](images/normaltriangle.png)

This is the flipped triangle :

![
    Just a flipped triangle
](images/invertedtriangle.png)

This effect was achieved by putting a `-` in front of the `x` and `y` coordinates in :

```rust
void main()
{
    gl_Position = vec4(position.x, position.y, position.z, 1.0f);
}
```

This way, we flip the triangle vertically (`x`) and horizontally (`y`).

**ii) Change the colour of the drawn triangle(s) to a different colour.**

The color of the triangle is set by this line in the *simple.frag* file :

```rust
void main()
{
    color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
```

The parameters are the RGB components of color, and an alpha parameter.
To modify color, I modified the vector to impact the RGB components :

```rust
vec4(0.0f, 1.0f, 0.0f, 1.0f);
```

The result, as we could expect from leaving the G component, is a green triangle :

![
    Just a green triangle
](images/greentriangle.png)

**END**