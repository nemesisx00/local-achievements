mod app;
mod appdata;
mod retroachievement;
mod steamachievement;
mod game;

use ::godot::engine::Node;
use ::godot::obj::Gd;

/**
Iterate over a node's children, calling queue_free() on each of them.

Parameter | Description
---|---
node | The `Node` whose children are to be freed.
*/
pub fn freeChildren(node: &mut Gd<Node>)
{
	if node.get_child_count() > 0
	{
		for i in (0..node.get_child_count()).rev()
		{
			if let Some(c) = node.get_child(i).as_mut()
			{
				//Change the node name just to avoid collisions if nodes are re-added before this one is freed.
				//Kind of a hack, but whatever.
				c.set_name("GoingToFree".into());
				c.queue_free();
			}
		}
	}
}
