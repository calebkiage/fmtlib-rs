#ifndef FFI_SHIM_INCLUDED
#define FFI_SHIM_INCLUDED
#include <fmt/args.h>
#include <fmt/core.h>
#include <memory>
#include "fmtlib/src/ffi.rs.h"
#include "rust/cxx.h"
#include <iostream>

namespace shimcpp {
    template<typename CB>
    void add_value(const shimrs::Value &v, CB callback) {
        if (v.is_bool()) {
            callback(v.get_bool());
        } else if (v.is_int64()) {
            callback(v.get_int64());
        } else if (v.is_float64()) {
            callback(v.get_float64());
        } else if (v.is_string()) {
            auto v0 = v.get_string();
            ::fmt::string_view sv(v0.data(), v0.size());
            callback(sv);
        }
    }

    rust::String format(const char* fmt, rust::Slice<const ::shimrs::Arg> args) {
        ::fmt::dynamic_format_arg_store<::fmt::format_context> store;
        for (auto& a : args) {
            auto &v = a.get_value();
            if (a.is_named()) {
                const char *name = static_cast<const char*>(a.get_name_ptr());
                auto cb = [&name, &store](auto data) {
                    store.push_back(::fmt::arg(name, data));
                };
                add_value(v, cb);
            } else {
                auto cb = [&store](auto data) {
                    store.push_back(data);
                };
                add_value(v, cb);
            }
        }

        return fmt::vformat(fmt, store);
    }
}
#endif