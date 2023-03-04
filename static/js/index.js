/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./client/index.js":
/*!*************************!*\
  !*** ./client/index.js ***!
  \*************************/
/***/ (() => {

eval("function _typeof(obj) { \"@babel/helpers - typeof\"; return _typeof = \"function\" == typeof Symbol && \"symbol\" == typeof Symbol.iterator ? function (obj) { return typeof obj; } : function (obj) { return obj && \"function\" == typeof Symbol && obj.constructor === Symbol && obj !== Symbol.prototype ? \"symbol\" : typeof obj; }, _typeof(obj); }\nfunction _defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if (\"value\" in descriptor) descriptor.writable = true; Object.defineProperty(target, _toPropertyKey(descriptor.key), descriptor); } }\nfunction _createClass(Constructor, protoProps, staticProps) { if (protoProps) _defineProperties(Constructor.prototype, protoProps); if (staticProps) _defineProperties(Constructor, staticProps); Object.defineProperty(Constructor, \"prototype\", { writable: false }); return Constructor; }\nfunction _toPropertyKey(arg) { var key = _toPrimitive(arg, \"string\"); return _typeof(key) === \"symbol\" ? key : String(key); }\nfunction _toPrimitive(input, hint) { if (_typeof(input) !== \"object\" || input === null) return input; var prim = input[Symbol.toPrimitive]; if (prim !== undefined) { var res = prim.call(input, hint || \"default\"); if (_typeof(res) !== \"object\") return res; throw new TypeError(\"@@toPrimitive must return a primitive value.\"); } return (hint === \"string\" ? String : Number)(input); }\nfunction _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError(\"Cannot call a class as a function\"); } }\n// Rusty Road\nvar RustyRoad = /*#__PURE__*/_createClass(function RustyRoad() {\n  _classCallCheck(this, RustyRoad);\n  this.name = \"nginx_admin\";\n  this.greet = function () {\n    console.log(\"Welcome to \".concat(this.name, \" powered by Rusty Road\"));\n  };\n});\nvar rusty_road = new RustyRoad();\nrusty_road.greet();\ndocument.getElementById(\"reload-nginx\").addEventListener(\"click\", function () {\n  fetch(\"/reload_nginx\", {\n    method: \"GET\",\n    headers: {\n      \"Content-Type\": \"application/json\"\n    }\n  }).then(function (response) {\n    return response.json();\n  }).then(function (data) {\n    console.log(data);\n  }).catch(function (error) {\n    console.error(error);\n  });\n});\ndocument.getElementById(\"stop-nginx\").addEventListener(\"click\", function () {\n  fetch(\"/stop_nginx\", {\n    method: \"GET\",\n    headers: {\n      \"Content-Type\": \"application/json\"\n    }\n  }).then(function (response) {\n    return response.json();\n  }).then(function (data) {\n    console.log(data);\n  }).catch(function (error) {\n    console.error(error);\n  });\n});\ndocument.getElementById(\"start-nginx\").addEventListener(\"click\", function () {\n  fetch(\"/start_nginx\", {\n    method: \"GET\",\n    headers: {\n      \"Content-Type\": \"application/json\"\n    }\n  }).then(function (response) {\n    return response.json();\n  }).then(function (data) {\n    console.log(data);\n  }).catch(function (error) {\n    console.error(error);\n  });\n});\ndocument.getElementById(\"vhost-button\").addEventListener(\"click\", function (e) {\n  e.preventDefault();\n  // get the form data\n  var formData = {\n    'domain': document.getElementById('vhost-domain').value\n  };\n\n  // send the form data to /create_vhost/{domain}\n  fetch('/create_vhost/' + formData.domain, {\n    method: 'GET',\n    headers: {\n      'Content-Type': 'application/json'\n    }\n  }).then(function (response) {\n    return response.json();\n  }).then(function (data) {\n    console.log(data);\n  });\n});\n\n//# sourceURL=webpack://rustyrocket/./client/index.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = {};
/******/ 	__webpack_modules__["./client/index.js"]();
/******/ 	
/******/ })()
;