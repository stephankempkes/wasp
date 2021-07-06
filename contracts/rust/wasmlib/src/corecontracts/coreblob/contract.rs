// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

// @formatter:off

#![allow(dead_code)]

use std::ptr;

use crate::*;
use crate::corecontracts::coreblob::*;

pub struct StoreBlobCall {
    pub func:    ScFunc,
    pub params:  MutableStoreBlobParams,
    pub results: ImmutableStoreBlobResults,
}

impl StoreBlobCall {
    pub fn new(_ctx: &ScFuncContext) -> StoreBlobCall {
        let mut f = StoreBlobCall {
            func:    ScFunc::new(HSC_NAME, HFUNC_STORE_BLOB),
            params:  MutableStoreBlobParams { id: 0 },
            results: ImmutableStoreBlobResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
        f
    }
}

pub struct GetBlobFieldCall {
    pub func:    ScView,
    pub params:  MutableGetBlobFieldParams,
    pub results: ImmutableGetBlobFieldResults,
}

impl GetBlobFieldCall {
    pub fn new(_ctx: &ScFuncContext) -> GetBlobFieldCall {
        let mut f = GetBlobFieldCall {
            func:    ScView::new(HSC_NAME, HVIEW_GET_BLOB_FIELD),
            params:  MutableGetBlobFieldParams { id: 0 },
            results: ImmutableGetBlobFieldResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
        f
    }

    pub fn new_from_view(_ctx: &ScViewContext) -> GetBlobFieldCall {
        GetBlobFieldCall::new(&ScFuncContext {})
    }
}

pub struct GetBlobInfoCall {
    pub func:    ScView,
    pub params:  MutableGetBlobInfoParams,
    pub results: ImmutableGetBlobInfoResults,
}

impl GetBlobInfoCall {
    pub fn new(_ctx: &ScFuncContext) -> GetBlobInfoCall {
        let mut f = GetBlobInfoCall {
            func:    ScView::new(HSC_NAME, HVIEW_GET_BLOB_INFO),
            params:  MutableGetBlobInfoParams { id: 0 },
            results: ImmutableGetBlobInfoResults { id: 0 },
        };
        f.func.set_ptrs(&mut f.params.id, &mut f.results.id);
        f
    }

    pub fn new_from_view(_ctx: &ScViewContext) -> GetBlobInfoCall {
        GetBlobInfoCall::new(&ScFuncContext {})
    }
}

pub struct ListBlobsCall {
    pub func:    ScView,
    pub results: ImmutableListBlobsResults,
}

impl ListBlobsCall {
    pub fn new(_ctx: &ScFuncContext) -> ListBlobsCall {
        let mut f = ListBlobsCall {
            func:    ScView::new(HSC_NAME, HVIEW_LIST_BLOBS),
            results: ImmutableListBlobsResults { id: 0 },
        };
        f.func.set_ptrs(ptr::null_mut(), &mut f.results.id);
        f
    }

    pub fn new_from_view(_ctx: &ScViewContext) -> ListBlobsCall {
        ListBlobsCall::new(&ScFuncContext {})
    }
}

// @formatter:on
