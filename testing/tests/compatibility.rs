// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

use imagezero::*;

    #[test]
    fn it_works() {
        compress::doit();
        assert_eq!(2 + 2, 4);
    }
}