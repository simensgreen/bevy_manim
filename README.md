# Bevy Manim
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

[bevy][bevy] plugin for drawing and animation
## Motivation
I was inspired to create this plugin by the work 
of 3b1b [manim][manim]
and the need for bevy to develop direct drawing tools. The plugin is in its infancy and could change 
dramatically, but the basic concept will remain the same.

It is also based on lyon, an alternative may be useful
## Current goals
   
   - More drawing primitives such as the:
        * Rectangle
        * Oval
        * Curve
        * Polygon
        * 3D shapes
        * ...
   - More animations
   - Documentation
   - Get rid of the terrible `Arc<Mutex<...>>` aka `PackedAnimation`
   - Find a way to make the interface more user-friendly for drawing and scheduling animations
   - Add complex animations (more than one effect is applied to the same picture at the same time)
   - Integration with user input, for example, to start a chain of animations when you press the spacebar (useful for presentations)
   
## Version
| bevy | bevy_manim        |
| ---- | ----------------- |
| main | main              |
| v0.5 | v0.1              |

[bevy]: https://github.com/bevyengine/bevy
[manim]: https://github.com/3b1b/manim/tree/846c10a0ffb0625b2d4e0a0703b9586d6f8fc485