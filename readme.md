# Joshua's Graphics Engine

work in progress

Todo list:
- try to remove the implementation with traits and Arc as they are slow at this scale. Idea: make hittables and materials structs, and initialize their functions in respective constructors. Not sure how this will turn out...
- remove options and pass mutable values around
- generalize movable objects, instead of having a simple moving sphere
- add support to build mesh from .ply file
