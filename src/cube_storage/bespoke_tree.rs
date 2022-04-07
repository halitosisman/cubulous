use bevy::prelude::*;
use crate::cube_storage;
use crate::cube_storage::constants::*;
use crate::cube_storage::math::*;
use crate::cube_storage::loader::*;
use crate::cube_storage::types::*;

/// An array of cubes
struct BespokeLeaf {

    cubes: [CubeT; CUBES_IN_LEAF],
    cached: bool
}

impl BespokeLeaf {
    fn load(p: Point3d) {

    }
}

struct BespokeTwig {
    leaves: [*mut BespokeLeaf; LEAVES_ON_TWIG],
    cached: bool
}

struct BespokeBranch {
    twigs: [*mut BespokeTwig; TWIGS_ON_BRANCH],
    cached: bool
}

struct BespokeRoot {
    branches: [*mut BespokeBranch; BRANCHES_ON_ROOT],
    cached: bool
}