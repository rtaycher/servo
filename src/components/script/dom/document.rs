/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::DocumentBinding;
use dom::bindings::codegen::DocumentBinding::VisibilityState;
use dom::bindings::codegen::DocumentBinding::VisibilityStateValues::Visible;
use dom::bindings::utils::{DOMString, WrapperCache, ErrorResult, null_string};
use dom::bindings::utils::{BindingObject, CacheableWrapper};
use dom::element::{HTMLHtmlElement, HTMLHtmlElementTypeId, Element};
use dom::event::Event_;
use dom::htmlcollection::HTMLCollection;
use dom::node::{AbstractNode, ScriptView, Node};
use dom::window::Window;
use dom::windowproxy::WindowProxy;

use js::JSPROP_ENUMERATE;
use js::glue::*;
use js::jsapi::{JS_AddObjectRoot, JS_RemoveObjectRoot, JSObject, JSContext};
use servo_util::tree::{TreeNodeRef, TreeUtils};

use std::cast;
use std::ptr;
use std::str::eq_slice;

pub struct Document {
    root: AbstractNode<ScriptView>,
    wrapper: WrapperCache,
    window: Option<@mut Window>,
}

pub fn Document(root: AbstractNode<ScriptView>, window: Option<@mut Window>) -> @mut Document {
    unsafe {
        let doc = @mut Document {
            root: root,
            wrapper: WrapperCache::new(),
            window: window
        };
        let compartment = (*window.get_ref().page).js_info.get_ref().js_compartment;
        do root.with_base |base| {
            assert!(base.wrapper.get_wrapper().is_not_null());
            let rootable = base.wrapper.get_rootable();
            JS_AddObjectRoot(compartment.cx.ptr, rootable);
        }

        let cx = (*window.get_ref().page).js_info.get_ref().js_compartment.cx.ptr;
        doc.wrap_object_shared(cx, ptr::null()); //XXXjdm a proper scope would be nice

        match window {
            Some(win) => {
                //FIXME: This is a hack until Window is autogenerated
                let compartment = (*win.page).js_info.get_ref().js_compartment;
                compartment.define_property(~"document",
                                            RUST_OBJECT_TO_JSVAL(doc.wrapper.wrapper),
                                            GetJSClassHookStubPointer(PROPERTY_STUB) as *u8,
                                            GetJSClassHookStubPointer(STRICT_PROPERTY_STUB) as *u8,
                                            JSPROP_ENUMERATE);
            }
            None => ()
        }
        doc
    }
}

impl CacheableWrapper for Document {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        unsafe { cast::transmute(&self.wrapper) }
    }

    fn wrap_object_shared(@mut self, cx: *JSContext, scope: *JSObject) -> *JSObject {
        let mut unused = false;
        DocumentBinding::Wrap(cx, scope, self, &mut unused)
    }
}

impl BindingObject for Document {
    fn GetParentObject(&self, _cx: *JSContext) -> @mut CacheableWrapper {
        match self.window {
            Some(win) => win as @mut CacheableWrapper,
            None => fail!("whoops")
        }
    }
}

impl Document {
    pub fn Constructor(_owner: @mut Window, _rv: &mut ErrorResult) -> @mut Document {
        let root = ~HTMLHtmlElement {
            parent: Element::new(HTMLHtmlElementTypeId, ~"html")
        };

        let cx = _owner.page.js_info.get_ref().js_compartment.cx.ptr;
        let root = unsafe { Node::as_abstract_node(cx, root) };
        Document(root, None)
    }

    pub fn URL(&self) -> DOMString {
        null_string
    }

    pub fn DocumentURI(&self) -> DOMString {
        null_string
    }

    pub fn CompatMode(&self) -> DOMString {
        null_string
    }

    pub fn CharacterSet(&self) -> DOMString {
        null_string
    }

    pub fn ContentType(&self) -> DOMString {
        null_string
    }

    pub fn GetDocumentElement(&self) -> Option<AbstractNode<ScriptView>> {
        Some(self.root)
    }

    pub fn GetElementsByTagName(&self, tag: DOMString) -> @mut HTMLCollection {
        let mut elements = ~[];
        let tag = tag.to_str();
        let _ = for self.root.traverse_preorder |child| {
            if child.is_element() {
                do child.with_imm_element |elem| {
                    if elem.tag_name == tag {
                        elements.push(child);
                    }
                }
            }
        };
        let win = self.window.get_ref();
        let cx = win.page.js_info.get_ref().js_compartment.cx.ptr;
        let cache = win.get_wrappercache();
        let scope = cache.get_wrapper();
        HTMLCollection::new(elements, cx, scope)
    }

    pub fn GetElementsByTagNameNS(&self, _ns: DOMString, _tag: DOMString) -> @mut HTMLCollection {
        let win = self.window.get_ref();
        let cx = win.page.js_info.get_ref().js_compartment.cx.ptr;
        let cache = win.get_wrappercache();
        let scope = cache.get_wrapper();
        HTMLCollection::new(~[], cx, scope)
    }

    pub fn GetElementsByClassName(&self, _class: DOMString) -> @mut HTMLCollection {
        let win = self.window.get_ref();
        let cx = win.page.js_info.get_ref().js_compartment.cx.ptr;
        let cache = win.get_wrappercache();
        let scope = cache.get_wrapper();
        HTMLCollection::new(~[], cx, scope)

    }

    pub fn GetElementById(&self, _id: DOMString) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn CreateElement(&self, _local_name: DOMString, _rv: &mut ErrorResult) -> AbstractNode<ScriptView> {
        fail!("stub")
    }

    pub fn CreateElementNS(&self, _namespace: DOMString, _qualified_name: DOMString, _rv: &mut ErrorResult) -> AbstractNode<ScriptView> {
        fail!("stub")
    }

    pub fn CreateEvent(&self, _interface: DOMString, _rv: &mut ErrorResult) -> @mut Event_ {
        fail!("stub")
    }

    pub fn GetInputEncoding(&self) -> DOMString {
        null_string
    }

    pub fn Referrer(&self) -> DOMString {
        null_string
    }

    pub fn LastModified(&self) -> DOMString {
        null_string
    }

    pub fn ReadyState(&self) -> DOMString {
        null_string
    }

    pub fn Title(&self) -> DOMString {
        null_string
    }

    pub fn SetTitle(&self, _title: DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Dir(&self) -> DOMString {
        null_string
    }

    pub fn SetDir(&self, _dir: DOMString) {
    }

    pub fn GetDefaultView(&self) -> Option<@mut WindowProxy> {
        None
    }

    pub fn GetActiveElement(&self) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn HasFocus(&self, _rv: &mut ErrorResult) -> bool {
        false
    }

    pub fn GetCurrentScript(&self) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn ReleaseCapture(&self) {
    }

    pub fn MozFullScreenEnabled(&self) -> bool {
        false
    }

    pub fn GetMozFullScreenElement(&self, _rv: &mut ErrorResult) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn GetMozPointerLockElement(&self) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn MozExitPointerLock(&self) {
    }

    pub fn Hidden(&self) -> bool {
        false
    }

    pub fn MozHidden(&self) -> bool {
        self.Hidden()
    }

    pub fn VisibilityState(&self) -> VisibilityState {
        Visible
    }

    pub fn MozVisibilityState(&self) -> VisibilityState {
        self.VisibilityState()
    }

    pub fn GetSelectedStyleSheetSet(&self) -> DOMString {
        null_string
    }

    pub fn SetSelectedStyleSheetSet(&self, _sheet: DOMString) {
    }

    pub fn GetLastStyleSheetSet(&self) -> DOMString {
        null_string
    }

    pub fn GetPreferredStyleSheetSet(&self) -> DOMString {
        null_string
    }

    pub fn EnableStyleSheetsForSet(&self, _name: DOMString) {
    }

    pub fn ElementFromPoint(&self, _x: f32, _y: f32) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn QuerySelector(&self, _selectors: DOMString, _rv: &mut ErrorResult) -> Option<AbstractNode<ScriptView>> {
        None
    }

    pub fn GetElementsByName(&self, name: DOMString) -> @mut HTMLCollection {
        let mut elements = ~[];
        let name = name.to_str();
        let _ = for self.root.traverse_preorder |child| {
            if child.is_element() {
                do child.with_imm_element |elem| {
                    match elem.get_attr("name") {
                        Some(val) => if eq_slice(val, name) { elements.push(child) },
                        None() => ()
                    }
                }
            }
        };
        let win = self.window.get_ref();
        let cx = win.page.js_info.get_ref().js_compartment.cx.ptr;
        let cache = win.get_wrappercache();
        let scope = cache.get_wrapper();
        HTMLCollection::new(elements, cx, scope)
    }

    pub fn content_changed(&self) {
        for self.window.iter().advance |window| {
            window.content_changed()
        }
    }

    pub fn teardown(&self) {
        unsafe {
            let compartment = (*self.window.get_ref().page).js_info.get_ref().js_compartment;
            do self.root.with_base |node| {
                assert!(node.wrapper.get_wrapper().is_not_null());
                let rootable = node.wrapper.get_rootable();
                JS_RemoveObjectRoot(compartment.cx.ptr, rootable);
            }
        }
    }
}

