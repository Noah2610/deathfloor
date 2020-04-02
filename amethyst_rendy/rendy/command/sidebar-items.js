initSidebarItems({"fn":[["families_from_device","Query queue families from device."]],"struct":[["CommandBuffer","Command buffer wrapper. This wrapper defines state with usage, level and ability to be individually reset at type level. This way many methods become safe."],["CommandPool","Simple pool wrapper. Doesn't provide any guarantees. Wraps raw buffers into `CommandCommand buffer`."],["Compute","Capable of compute commands execution."],["DispatchCommand","Draw command for dispatch."],["DrawCommand","Draw command for [`draw_indirect`]."],["DrawIndexedCommand","Draw command for [`draw_indexed_indirect`]."],["Encoder","Trait to encode commands outside render pass."],["EncoderCommon","Encoder for recording commands inside or outside renderpass."],["ExecutableState","Command buffer in executable state can be submitted."],["Execute","Capable of either compute or graphics commands execution."],["Families","Collection of queue families of one device."],["Family","Family of the command queues. Queues from one family can share resources and execute command buffers associated with the family. All queues of the family have same capabilities."],["FamilyId","Family id."],["Fence","Fence wrapper."],["FenceEpoch","Queue epoch is the point in particluar queue timeline when fence is submitted."],["General","Capable of any commands execution."],["Graphics","Capable of graphics command execution."],["IndividualReset","This flag specify that buffer can be reset individually."],["InitialState","Command buffer state in which all buffers start. Resetting also moves buffer to this state."],["InvalidState","One-shot buffers move to invalid state after execution. Invalidating any resource referenced in any command recorded to the buffer implicitly move it to the invalid state."],["MultiShot","Command buffer with this usage flag will move back to executable state after execution."],["NoIndividualReset","This flag specify that buffer cannot be reset individually."],["NoSimultaneousUse","Additional flag that disallows resubmission of a command buffer while it is still in a pending state It must be completed, i.e. a fence must submitted with this buffer or later into the same queue and be waited on before buffer resubmission. `Submit<B, NoSimultaneousUse>` cannot be submitted more than once."],["OneShot","Command buffer with this usage flag will move to invalid state after execution. Resubmitting will require reset and rerecording commands."],["OutsideRenderPass","Primary buffers must has this flag as they cannot has `RenderPassContinue` flag. Secondary buffers with this usage flag cannot be executed as part of render-pass."],["PendingState","Command buffer in pending state are submitted to the device. Command buffer in pending state must never be invalidated or reset because device may read it at the moment. Proving device is done with buffer requires nontrivial strategies. Therefore moving buffer from pending state requires `unsafe` method."],["PrimaryLevel","Command buffers of this level can be submitted to the command queues."],["Queue","Command queue wrapper."],["QueueId","Queue id."],["RecordingState","Command buffer in recording state could be populated with commands."],["RenderPassContinue","Buffers with this usage flag must be secondary buffers executed entirely in render-pass."],["RenderPassEncoder","Special encoder to record render-pass commands."],["RenderPassInlineEncoder","Special encoder to record commands inside render pass."],["RenderPassSecondaryEncoder","Special encoder to execute secondary buffers inside render pass."],["SecondaryLevel","Command buffers of this level can be executed as part of the primary buffers."],["SimultaneousUse","Additional flag that allows resubmission of a command buffer while it is still in a pending state. `Submit<B, SimultaneousUse>` can be submitted more than once."],["Submission","Command queue submission."],["Submit","Structure contains command buffer ready for submission."],["Transfer","Capable of transfer only."]],"trait":[["BeginInfo","Begin info for specific level and render pass relation."],["Capability","Abstract capability specifier."],["Level","Type-level buffer level flag. It defines whether buffer can be submitted to the command queues or executed as part of the primary buffers."],["RenderPassRelation","Trait implemented for type-level render pass relation flags. `RenderPassContinue` and `OutsideRenderPass`."],["Reset","Specify flags required for command pool creation to allow individual buffer reset."],["Resettable","States in which command buffer can de reset."],["Submittable","Submittable object. Values that implement this trait can be submitted to the queues or executed as part of primary buffers (in case of `Submittable<B, SecondaryLevel>`)."],["Supports","Check if capability supported."],["Usage","Type-level usage flags. It defines if buffer can be resubmitted without reset. Or even resubmitted while being executed."]],"type":[["PendingOnceState","Command buffer in pending state are submitted to the device. Command buffer in pending state must never be invalidated or reset because device may read it at the moment. Proving device is done with buffer requires nontrivial strategies. Therefore moving buffer from pending state requires `unsafe` method. This type alias can be used for one-shot command buffers."]]});