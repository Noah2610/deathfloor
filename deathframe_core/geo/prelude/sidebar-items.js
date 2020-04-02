initSidebarItems({"enum":[["Axis","Just a plain `Axis` enum with `X` and `Y` variants."]],"struct":[["AxisIter","An iterator over both axes."],["Rect","A `Rect` is simply an area. It has positions bounding sides (top, bottom, left, right)."],["RectBuilder","Builder for `Rect`."]],"trait":[["ByAxis","Anything implementing the `ByAxis` trait, returns an item through the `by_axis` method, by passing an `&Axis`. This is useful when you have a tuple or similar with two items, where each item represents an axis. You can get the specifc item by indexing with an `Axis`. Here's an example, and what code this would save: ``` use deathframe_core::geo::prelude::{Axis, ByAxis};"]]});