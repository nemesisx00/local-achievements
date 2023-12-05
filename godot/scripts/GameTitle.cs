using Godot;

namespace LocalAchievements;

public partial class GameTitle : Label
{
	private static readonly NodePath TabsPath = new("../Tabs");
	private static readonly Vector2 MinimumSize = new(0, 648);
	
	private MarginContainer outerContainer;
	private TabContainer tabs;

	public override void _Input(InputEvent evt)
	{
		if(evt is InputEventMouseButton iemb && iemb.Pressed && iemb.ButtonIndex == MouseButton.Left)
			handleClick();
	}
	
	public override void _Ready()
	{
		tabs = GetNode<TabContainer>(TabsPath);
		outerContainer = GetParent().GetParent<MarginContainer>();
	}
	
	private void handleClick()
	{
		if(tabs.Visible)
		{
			tabs.Hide();
			outerContainer.CustomMinimumSize = Vector2.Zero;
		}
		else
		{
			tabs.Show();
			outerContainer.CustomMinimumSize = MinimumSize;
		}
	}
}
