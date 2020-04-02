initSidebarItems({"mod":[["gather","Helper gatherer structures for collecting information about the world."]],"struct":[["DynamicUniform","Provides per-image abstraction for an arbitrary `DescriptorSet`."],["DynamicVertexData","This structure wraps [PerImageDynamicVertexData], managing multiple instances and providing an easy-to-use interface for having per-image buffers. This is needed because multiple images (frames) can be in flight at any given time, so multiple buffers are needed for the same data."],["EnvironmentSub","Submodule for loading and binding descriptor sets for a 3D, lit environment. This also abstracts away the need for handling multiple images in flight, as it provides per-image submissions."],["FlatEnvironmentSub","Submodule for loading and binding descriptor sets for a flat, unlit environment. This also abstracts away the need for handling multiple images in flight, as it provides per-image submissions."],["IndexData","Type used to compile-time specify the type of vertex buffer data managed by a  `DynamicVertexData`"],["MaterialId","Material ID newtype, preventing users from creating arbitrary `MaterialId`. Represented as a `u32`"],["MaterialSub","Material helper submodule for allocating and binding materials and their associated textures."],["SkinningSub","Provides per-image abstraction for submitting skinned mesh skeletal information."],["TextureId","Texture ID newtype, preventing users from creating arbitrary `TextureId`. Represented as a `u32`"],["TextureSub","Texture helper submodule for allocating and binding textures and abstracting per-image submissions."],["VertexData","Type used to compile-time specify the type of vertex buffer data managed by a  `DynamicVertexData`"]],"trait":[["VertexDataBufferType","Type trait for allowing type-based implementation details for binding the different buffer types of index and vertex `DynamicVertexData`"]],"type":[["DynamicIndexBuffer","Type alias for a set of dynamic index buffer data to be managed. See the documentation for [DynamicVertexData] for implementation details."],["DynamicVertexBuffer","Type alias for a set of dynamic vertex buffer data to be managed. See the documentation for [DynamicVertexData] for implementation details."]]});