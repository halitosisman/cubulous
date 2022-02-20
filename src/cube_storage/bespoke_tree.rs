use bevy::prelude::*;
use crate::cube_storage;
use crate::cube_storage::cube::Cube;
use crate::cube_storage::constants::*;

struct BespokeLeaf {
    cubes: [u32; CUBES_IN_LEAF],
    cached: bool
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