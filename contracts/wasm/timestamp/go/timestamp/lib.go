// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the json schema instead

package timestamp

import "github.com/iotaledger/wasp/packages/vm/wasmlib/go/wasmlib"

func OnLoad() {
	exports := wasmlib.NewScExports()
	exports.AddFunc(FuncNow, funcNowThunk)
	exports.AddView(ViewGetTimestamp, viewGetTimestampThunk)
}

type NowContext struct {
	State MutabletimestampState
}

func funcNowThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("timestamp.funcNow")
	f := &NowContext{
		State: MutabletimestampState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	funcNow(ctx, f)
	ctx.Log("timestamp.funcNow ok")
}

type GetTimestampContext struct {
	Results MutableGetTimestampResults
	State   ImmutabletimestampState
}

func viewGetTimestampThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("timestamp.viewGetTimestamp")
	results := wasmlib.NewScDict()
	f := &GetTimestampContext{
		Results: MutableGetTimestampResults{
			proxy: results.AsProxy(),
		},
		State: ImmutabletimestampState{
			proxy: wasmlib.NewStateProxy(),
		},
	}
	viewGetTimestamp(ctx, f)
	ctx.Log("timestamp.viewGetTimestamp ok")
	ctx.Results(results)
}
