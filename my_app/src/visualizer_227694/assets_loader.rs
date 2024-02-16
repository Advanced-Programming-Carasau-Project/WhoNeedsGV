use bevy::prelude::*;
#[derive(Resource,Debug,Default)]
pub struct SceneAssets{
    pub robot:Handle<Scene>,
    pub grass:Handle<Scene>,
    pub sand:Handle<Scene>,
    pub street:Handle<Scene>,
    pub snow:Handle<Scene>,
    pub deep_water:Handle<Scene>,
    pub shallow_water:Handle<Scene>,
    pub hill:Handle<Scene>,
    pub mountain:Handle<Scene>,
    pub lava:Handle<Scene>,
    pub teleport:Handle<Scene>,
    pub wall:Handle<Scene>,
    pub rock1:Handle<Scene>,
    pub rock2:Handle<Scene>,
    pub rock3:Handle<Scene>,
    pub tree1:Handle<Scene>,
    pub tree2:Handle<Scene>,
    pub tree3:Handle<Scene>,
    pub garbage:Handle<Scene>,
    pub fire:Handle<Scene>,
    pub coin:Handle<Scene>,
    pub bin:Handle<Scene>,
    pub crate_:Handle<Scene>,
    pub bank:Handle<Scene>,
    pub market:Handle<Scene>,
    pub fish:Handle<Scene>,
    pub building:Handle<Scene>,
    pub bush:Handle<Scene>,
    pub jolly_block:Handle<Scene>,
    pub mirto:Handle<Scene>,
    pub scarecrow:Handle<Scene>,
}
#[derive(Resource,Debug,Default)]
pub struct ImageAssets{
    pub coin:Handle<Image>,
    pub tree:Handle<Image>,
    pub rock:Handle<Image>,
    pub scarecrow:Handle<Image>,
    pub chicken:Handle<Image>,
    pub garbage:Handle<Image>,
    pub jolly_block:Handle<Image>,
    pub mirto:Handle<Image>,
    pub fish:Handle<Image>,
    pub water:Handle<Image>,
    pub bush:Handle<Image>,
    pub energy_border:Handle<Image>,
    pub energy:Handle<Image>,
    pub points_border:Handle<Image>,
    pub points:Handle<Image>,
    pub back_pack:Handle<Image>,
    pub sunny:Handle<Image>,
    pub night:Handle<Image>,
    pub rainy:Handle<Image>,
    pub rainy_night:Handle<Image>,
    pub trentino_snow:Handle<Image>,
    pub trentino_snow_night:Handle<Image>,
    pub tropical_monson:Handle<Image>,
    pub tropical_monson_night:Handle<Image>,
    pub foggy:Handle<Image>,
    pub foggy_night:Handle<Image>,
}
pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(PreStartup,load_assets)
            .init_resource::<ImageAssets>().add_systems(PreStartup,load_images);
    }
}
pub fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>){
    *scene_assets = SceneAssets{
        robot:asset_server.load("robot.glb#Scene0"),
        grass:asset_server.load("grass.glb#Scene0"),
        sand:asset_server.load("sand.glb#Scene0"),
        street:asset_server.load("street.glb#Scene0"),
        snow:asset_server.load("snow.glb#Scene0"),
        deep_water:asset_server.load("deep_water2.glb#Scene0"),
        shallow_water:asset_server.load("shallow_water2.glb#Scene0"),
        hill:asset_server.load("hill.glb#Scene0"),
        mountain:asset_server.load("mountain.glb#Scene0"),
        lava:asset_server.load("lava2.glb#Scene0"),
        teleport:asset_server.load("teleport.glb#Scene0"),
        wall:asset_server.load("wall.glb#Scene0"),
        rock1:asset_server.load("rock1.glb#Scene0"),
        rock2:asset_server.load("rock2.glb#Scene0"),
        rock3:asset_server.load("rock3.glb#Scene0"),
        tree1:asset_server.load("tree1.glb#Scene0"),
        tree2:asset_server.load("tree2.glb#Scene0"),
        tree3:asset_server.load("tree3.glb#Scene0"),
        garbage:asset_server.load("garbage.glb#Scene0"),
        fire:asset_server.load("fire.glb#Scene0"), //TODO animazioni? sarebbe figo
        coin:asset_server.load("coin.glb#Scene0"),
        bin:asset_server.load("bin.glb#Scene0"),
        crate_:asset_server.load("crate.glb#Scene0"),
        bank:asset_server.load("bank.glb#Scene0"),
        market:asset_server.load("market.glb#Scene0"),
        fish:asset_server.load("fish.glb#Scene0"),
        building:asset_server.load("building.glb#Scene0"),
        bush:asset_server.load("bush.glb#Scene0"),
        jolly_block:asset_server.load("jolly_block.glb#Scene0"),
        mirto:asset_server.load("mirto_jb_goldo.glb#Scene0"),
        scarecrow:asset_server.load("scarecrow.glb#Scene0"),
    }
}
pub fn load_images(mut image_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>){
    *image_assets = ImageAssets{
        coin:asset_server.load("textures/coin1.png"),
        tree:asset_server.load("textures/tree.png"),
        rock:asset_server.load("textures/rock.png"),
        scarecrow:asset_server.load("textures/scarecrow.png"),
        chicken:asset_server.load("textures/chicken.png"),
        garbage:asset_server.load("textures/garbage.png"),
        jolly_block:asset_server.load("textures/teleport.png"),
        mirto:asset_server.load("textures/mirto.png"),
        fish:asset_server.load("textures/fish.png"),
        water:asset_server.load("textures/water.png"),
        bush:asset_server.load("textures/bush.png"),
        energy_border:asset_server.load("textures/EnergyBorder.png"),
        energy:asset_server.load("textures/EnergyBox.png"),
        points_border:asset_server.load("textures/PointsBorder.png"),
        points:asset_server.load("textures/PointsBox.png"),
        back_pack:asset_server.load("textures/BackPack.png"),
        sunny:asset_server.load("textures/sunny.png"),
        night:asset_server.load("textures/night.png"),
        rainy:asset_server.load("textures/rainy.png"),
        rainy_night:asset_server.load("textures/rainy_night.png"),
        trentino_snow:asset_server.load("textures/trentino_snow.png"),
        trentino_snow_night:asset_server.load("textures/trentino_snow.png"),
        tropical_monson:asset_server.load("textures/tropical_monson.png"),
        tropical_monson_night:asset_server.load("textures/tropical_monson.png"),
        foggy:asset_server.load("textures/foggy.png"),
        foggy_night:asset_server.load("textures/foggy_night.png"),
    }
}