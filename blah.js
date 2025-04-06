!function(t) {
    var e = {};

    function n(o) {
        if (e[o])
            return e[o].exports;
        var r = e[o] = {
            i: o,
            l: !1,
            exports: {}
        };
        return t[o].call(r.exports, r, r.exports, n),
        r.l = !0,
        r.exports
    }
    n.m = t,
    n.c = e,
    n.d = function(t, e, o) {
        n.o(t, e) || Object.defineProperty(t, e, {
            enumerable: !0,
            get: o
        })
    }
    ,
    n.r = function(t) {
        "undefined" != typeof Symbol && Symbol.toStringTag && Object.defineProperty(t, Symbol.toStringTag, {
            value: "Module"
        }),
        Object.defineProperty(t, "__esModule", {
            value: !0
        })
    }
    ,
    n.t = function(t, e) {
        if (1 & e && (t = n(t)),
        8 & e)
            return t;
        if (4 & e && "object" == typeof t && t && t.__esModule)
            return t;
        var o = Object.create(null);
        if (n.r(o),
        Object.defineProperty(o, "default", {
            enumerable: !0,
            value: t
        }),
        2 & e && "string" != typeof t)
            for (var r in t)
                n.d(o, r, function(e) {
                    return t[e]
                }
                .bind(null, r));
        return o
    }
    ,
    n.n = function(t) {
        var e = t && t.__esModule ? function() {
            return t.default
        }
        : function() {
            return t
        }
        ;
        return n.d(e, "a", e),
        e
    }
    ,
    n.o = function(t, e) {
        return Object.prototype.hasOwnProperty.call(t, e)
    }
    ,
    n.p = "",
    n(n.s = 3)
}([function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i, s = n(1), a = n(20), l = n(22), d = n(23), h = n(24), u = n(25), f = n(2), p = n(26), c = n(27);
    !function(t) {
        t.PLAYING = "playing",
        t.WIN = "win",
        t.LOSE = "lose"
    }(i || (i = {}));
    var k = function(t) {
        function e(e, n, o) {
            var r = t.call(this, {
                key: "MainScene"
            }) || this;
            return r.w = e,
            r.h = n,
            r.r = o,
            r.dx = 2 * r.r,
            r.dy = r.r * Math.sqrt(3),
            r
        }
        return r(e, t),
        Object.defineProperty(e.prototype, "blocks", {
            get: function() {
                return this.data.get("blocks")
            },
            set: function(t) {
                this.data.set("blocks", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "blocksData", {
            get: function() {
                var t = [];
                return this.blocks.forEach(function(e, n) {
                    t[n] = [],
                    e.forEach(function(e, o) {
                        t[n][o] = e.isWall
                    })
                }),
                t
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "cat", {
            get: function() {
                return this.data.get("cat")
            },
            set: function(t) {
                this.data.set("cat", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "statusBar", {
            get: function() {
                return this.data.get("status_bar")
            },
            set: function(t) {
                this.data.set("status_bar", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "creditText", {
            get: function() {
                return this.data.get("credit_text")
            },
            set: function(t) {
                this.data.set("credit_text", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "state", {
            get: function() {
                return this.data.get("state")
            },
            set: function(t) {
                switch (t) {
                case i.PLAYING:
                    break;
                case i.LOSE:
                    this.setStatusText(f.default("You lose :("));
                    break;
                case i.WIN:
                    this.setStatusText(f.default("You won!"));
                    break;
                default:
                    return
                }
                this.data.set("state", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        e.getNeighbours = function(t, e) {
            var n, o, r, i, s = {
                i: t - 1,
                j: e
            }, a = {
                i: t + 1,
                j: e
            };
            0 == (1 & e) ? (n = {
                i: t - 1,
                j: e - 1
            },
            o = {
                i: t,
                j: e - 1
            },
            r = {
                i: t - 1,
                j: e + 1
            },
            i = {
                i: t,
                j: e + 1
            }) : (n = {
                i: t,
                j: e - 1
            },
            o = {
                i: t + 1,
                j: e - 1
            },
            r = {
                i: t,
                j: e + 1
            },
            i = {
                i: t + 1,
                j: e + 1
            });
            var l = [];
            return l[0] = s,
            l[1] = n,
            l[2] = o,
            l[3] = a,
            l[4] = i,
            l[5] = r,
            l
        }
        ,
        e.prototype.preload = function() {
            var t = this.r / s.default.catStepLength;
            for (var e in s.default.textures)
                this.load.addFile(new c.default(this.load,e,s.default.textures[e],{
                    scale: t
                }))
        }
        ,
        e.prototype.create = function() {
            this.createAnimations(),
            this.createBlocks(),
            this.createCat(),
            this.createStatusText(),
            this.createResetButton(),
            this.createCreditText(),
            this.reset(),
            this.game.solver && (this.cat.solver = this.game.solver)
        }
        ,
        e.prototype.getPosition = function(t, e) {
            return {
                x: 3 * this.r + (0 == (1 & e) ? this.r : this.dx) + t * this.dx,
                y: 3 * this.r + this.r + e * this.dy
            }
        }
        ,
        e.prototype.getBlock = function(t, e) {
            return t >= 0 && t < this.w && e >= 0 && e < this.h ? this.blocks[t][e] : null
        }
        ,
        e.prototype.playerClick = function(t, e) {
            if (this.cat.anims.isPlaying && this.cat.anims.stop(),
            this.state !== i.PLAYING)
                return this.setStatusText(f.default("")),
                this.reset(),
                !1;
            var n = this.getBlock(t, e);
            return n ? n.isWall ? (this.setStatusText(f.default("")),
            !1) : this.cat.i === t && this.cat.j === e ? (this.setStatusText(f.default("")),
            !1) : (n.isWall = !0,
            this.cat.isCaught() ? (this.setStatusText(f.default("")),
            this.state = i.WIN,
            !1) : (this.setStatusText(f.default("")),
            this.cat.step() || (this.setStatusText(f.default("")),
            this.state = i.WIN),
            !0)) : (this.setStatusText(f.default("")),
            !1)
        }
        ,
        e.prototype.reset = function() {
            this.cat.reset(),
            this.resetBlocks(),
            this.randomWall(),
            this.state = i.PLAYING,
            this.setStatusText(f.default(""))
        }
        ,
        e.prototype.setStatusText = function(t) {
            this.statusBar.setText(t)
        }
        ,
        e.prototype.createAnimations = function() {
            var t = this;
            s.default.animations.forEach(function(e) {
                var n = [];
                e.textures.forEach(function(t) {
                    n.push({
                        key: t,
                        frame: 0
                    })
                }),
                t.anims.create({
                    key: e.name,
                    frames: n,
                    frameRate: s.default.frameRate,
                    repeat: e.repeat
                })
            })
        }
        ,
        e.prototype.createBlocks = function() {
            for (var t = [], e = 0; e < this.w; e++) {
                t[e] = [];
                for (var n = 0; n < this.h; n++) {
                    var o = new l.default(this,e,n,.9 * this.r);
                    t[e][n] = o,
                    this.add.existing(o),
                    o.on("player_click", this.playerClick.bind(this))
                }
            }
            this.blocks = t
        }
        ,
        e.prototype.createCat = function() {
            var t = this
              , e = new a.default(this);
            e.on("escaped", function() {
                t.state = i.LOSE
            }),
            e.on("win", function() {
                t.state = i.WIN
            }),
            e.solver = p.default,
            this.cat = e,
            this.add.existing(e)
        }
        ,
        e.prototype.createStatusText = function() {
            var t = new h.default(this);
            this.statusBar = t,
            this.add.existing(t)
        }
        ,
        e.prototype.createResetButton = function() {
            var t = this
              , e = new d.default(this);
            this.add.existing(e),
            e.on("pointerup", function() {
                t.reset()
            })
        }
        ,
        e.prototype.createCreditText = function() {
            var t = new u.default(this);
            this.creditText = t,
            this.add.existing(t)
        }
        ,
        e.prototype.resetBlocks = function() {
            this.blocks.forEach(function(t) {
                t.forEach(function(t) {
                    t.isWall = !1
                })
            })
        }
        ,
        e.prototype.randomWall = function() {
            for (var t = 0; t < 8; t++) {
                var e = Math.floor(this.w * Math.random())
                  , n = Math.floor(this.h * Math.random());
                e === this.cat.i && n === this.cat.j || (this.getBlock(e, n).isWall = !0)
            }
        }
        ,
        e
    }(Phaser.Scene);
    e.default = k
}
, function(t, e, n) {
    "use strict";
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var o = n(5)
      , r = n(6)
      , i = n(7)
      , s = n(8)
      , a = n(9)
      , l = n(10)
      , d = n(11)
      , h = n(12)
      , u = n(13)
      , f = n(14)
      , p = n(15)
      , c = n(16)
      , k = n(17)
      , g = n(18)
      , w = n(19);
    e.default = {
        textures: {
            bottom_left_1: o,
            bottom_left_2: r,
            bottom_left_3: i,
            bottom_left_4: s,
            bottom_left_5: a,
            left_1: l,
            left_2: d,
            left_3: h,
            left_4: u,
            left_5: f,
            top_left_1: p,
            top_left_2: c,
            top_left_3: k,
            top_left_4: g,
            top_left_5: w
        },
        animations: [{
            name: "left_step",
            textures: ["left_1", "left_2", "left_3", "left_4", "left_5"],
            repeat: 0
        }, {
            name: "top_left_step",
            textures: ["top_left_1", "top_left_2", "top_left_3", "top_left_4", "top_left_5"],
            repeat: 0
        }, {
            name: "bottom_left_step",
            textures: ["bottom_left_1", "bottom_left_2", "bottom_left_3", "bottom_left_4", "bottom_left_5"],
            repeat: 0
        }, {
            name: "left_run",
            textures: ["left_2", "left_3", "left_4", "left_5"],
            repeat: 3
        }, {
            name: "top_left_run",
            textures: ["top_left_2", "top_left_3", "top_left_4", "top_left_5"],
            repeat: 3
        }, {
            name: "bottom_left_run",
            textures: ["bottom_left_2", "bottom_left_3", "bottom_left_4", "bottom_left_5"],
            repeat: 3
        }],
        origins: {
            left: {
                x: .75,
                y: .75
            },
            top_left: {
                x: .63,
                y: .83
            },
            bottom_left: {
                x: .65,
                y: .5
            }
        },
        stopTextures: {
            left: "left_1",
            top_left: "top_left_1",
            bottom_left: "bottom_left_1"
        },
        cannotEscapeTextures: {
            left: "left_2",
            top_left: "top_left_2",
            bottom_left: "bottom_left_2"
        },
        directions: [{
            scaleX: 1,
            name: "left"
        }, {
            scaleX: 1,
            name: "top_left"
        }, {
            scaleX: -1,
            name: "top_left"
        }, {
            scaleX: -1,
            name: "left"
        }, {
            scaleX: -1,
            name: "bottom_left"
        }, {
            scaleX: 1,
            name: "bottom_left"
        }],
        catDefaultDirection: 5,
        catStepLength: 20,
        frameRate: 15,
        translations: {}
    }
}
, function(t, e, n) {
    "use strict";
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var o = n(1);
    e.default = function(t) {
        var e = o.default.translations[t];
        return void 0 === e ? t : e
    }
}
, function(t, e, n) {
    "use strict";
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var o = n(4);
    window.CatchTheCatGame = o.default
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = n(0)
      , s = function(t) {
        function e(e) {
            var n = this;
            e.credit || (e.credit = ""),
            e.backgroundColor || (e.backgroundColor = 15658734);
            var o = e.w
              , r = e.h
              , s = e.r * window.devicePixelRatio
              , a = 1 / window.devicePixelRatio
              , l = Math.floor((6.5 + 2 * o) * s)
              , d = Math.floor((6 + Math.sqrt(3) * r) * s)
              , h = new i.default(o,r,s)
              , u = {
                width: l,
                height: d,
                type: Phaser.AUTO,
                parent: e.parent,
                backgroundColor: e.backgroundColor,
                scene: h,
                zoom: a
            };
            return (n = t.call(this, u) || this).myConfig = e,
            n.mainScene = h,
            n
        }
        return r(e, t),
        Object.defineProperty(e.prototype, "solver", {
            get: function() {
                return this._solver
            },
            set: function(t) {
                this._solver = t;
                try {
                    this.mainScene.cat.solver = t
                } finally {}
            },
            enumerable: !0,
            configurable: !0
        }),
        e
    }(Phaser.Game);
    /*!
     * Catch The Cat Game
     *
     * https://github.com/ganlvtech/phaser-catch-the-cat
     */
    e.default = s
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e) {
    t.exports = "too long"
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = n(0)
      , s = n(1)
      , a = n(21)
      , l = function(t) {
        function e(e) {
            var n = t.call(this, e, 0, 0, "__DEFAULT") || this;
            return n.on("animationrepeat", function() {
                n.moveForward()
            }),
            n.solver = a.default,
            n.direction = s.default.catDefaultDirection,
            n.reset(),
            n
        }
        return r(e, t),
        Object.defineProperty(e.prototype, "i", {
            get: function() {
                return this.getData("i")
            },
            set: function(t) {
                this.setData("i", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "j", {
            get: function() {
                return this.getData("j")
            },
            set: function(t) {
                this.setData("j", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "direction", {
            get: function() {
                return this.getData("direction")
            },
            set: function(t) {
                this.setData("direction", t),
                this.resetTextureToStop(),
                this.resetOriginAndScale()
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(e.prototype, "solver", {
            get: function() {
                return this.getData("solver")
            },
            set: function(t) {
                this.setData("solver", t)
            },
            enumerable: !0,
            configurable: !0
        }),
        e.prototype.reset = function() {
            this.anims.stop(),
            this.direction = s.default.catDefaultDirection,
            this.resetIJ()
        }
        ,
        e.prototype.step = function() {
            var t = this.solver.call(this, this.scene.blocksData, this.i, this.j);
            return t < 0 || t > 6 ? (this.caught(),
            !1) : !!this.stepDirection(t) || (this.caught(),
            !1)
        }
        ,
        e.prototype.isCaught = function() {
            var t = this;
            return !this.getCurrentNeighbours().some(function(e, n) {
                var o = t.scene.getBlock(e.i, e.j);
                return null !== o && !o.isWall
            })
        }
        ,
        e.prototype.caught = function() {
            this.setTexture(s.default.cannotEscapeTextures[s.default.directions[this.direction].name])
        }
        ,
        e.prototype.escape = function() {
            0 === this.j || this.j === this.scene.h - 1 ? this.runForward() : 0 === this.i ? this.runDirection(0) : this.i === this.scene.w - 1 && this.runDirection(3)
        }
        ,
        e.prototype.setIJ = function(t, e) {
            this.i = t,
            this.j = e;
            var n = this.scene.getPosition(t, e);
            return this.setPosition(n.x, n.y)
        }
        ,
        e.prototype.resetIJ = function() {
            this.setIJ(Math.floor(this.scene.w / 2), Math.floor(this.scene.h / 2))
        }
        ,
        e.prototype.isEscaped = function() {
            return this.i <= 0 || this.i >= this.scene.w - 1 || this.j <= 0 || this.j >= this.scene.h - 1
        }
        ,
        e.prototype.checkState = function() {
            this.isEscaped() ? (this.escape(),
            this.emit("escaped")) : this.isCaught() && (this.caught(),
            this.emit("win"))
        }
        ,
        e.prototype.getCurrentNeighbours = function() {
            return i.default.getNeighbours(this.i, this.j)
        }
        ,
        e.prototype.resetTextureToStop = function() {
            this.setTexture(s.default.stopTextures[s.default.directions[this.direction].name])
        }
        ,
        e.prototype.resetOriginAndScale = function() {
            var t = s.default.directions[this.direction]
              , e = s.default.origins[t.name];
            this.setOrigin(e.x, e.y),
            this.scaleX = t.scaleX
        }
        ,
        e.prototype.moveForward = function() {
            var t = this.getCurrentNeighbours()[this.direction];
            this.setIJ(t.i, t.j),
            this.checkState()
        }
        ,
        e.prototype.stepForward = function() {
            var t = this
              , e = this.getCurrentNeighbours()[this.direction]
              , n = this.scene.getBlock(e.i, e.j);
            return null !== n && (!n.isWall && (this.play(s.default.directions[this.direction].name + "_step"),
            this.once("animationcomplete", function() {
                t.moveForward(),
                t.resetTextureToStop()
            }),
            !0))
        }
        ,
        e.prototype.stepDirection = function(t) {
            return this.direction = t,
            this.stepForward()
        }
        ,
        e.prototype.runForward = function() {
            this.play(s.default.directions[this.direction].name + "_run")
        }
        ,
        e.prototype.runDirection = function(t) {
            this.direction = t,
            this.runForward()
        }
        ,
        e
    }(Phaser.GameObjects.Sprite);
    e.default = l
}
, function(t, e, n) {
    "use strict";
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var o = n(0);
    e.default = function(t, e, n) {
        var r = -1;
        return o.default.getNeighbours(e, n).forEach(function(e, n) {
            -1 === r && (void 0 === t[e.i] || void 0 === t[e.i][e.j] || t[e.i][e.j] || (r = n))
        }),
        r
    }
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = function(t) {
        function e(e, n, o, r) {
            var i = this
              , s = e.getPosition(n, o);
            (i = t.call(this, e, s.x, s.y, r, 0, 360, !1, 0, 1) || this).i = n,
            i.j = o,
            i.r = r,
            i.isWall = !1;
            var a = new Phaser.Geom.Circle(i.r / 2,i.r / 2,i.r);
            return i.setInteractive(a, Phaser.Geom.Circle.Contains),
            i.on("pointerdown", function() {
                i.emit("player_click", i.i, i.j)
            }),
            i
        }
        return r(e, t),
        Object.defineProperty(e.prototype, "isWall", {
            get: function() {
                return this._isWall
            },
            set: function(t) {
                this._isWall = t,
                this.fillColor = t ? 13158 : 11786751
            },
            enumerable: !0,
            configurable: !0
        }),
        e
    }(Phaser.GameObjects.Arc);
    e.default = i
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = n(2)
      , s = function(t) {
        function e(e) {
            var n = t.call(this, e, 0, 0, i.default("Restart"), {}) || this;
            n.setColor("#000000");
            var o = e.r;
            n.setFontSize(o),
            n.setPadding(o, o, o, o),
            n.setPosition(0, e.game.canvas.height),
            n.setOrigin(0, 1);
            var r = new Phaser.Geom.Rectangle(0,0,n.width,n.height);
            return n.setInteractive(r, Phaser.Geom.Rectangle.Contains),
            n
        }
        return r(e, t),
        e
    }(Phaser.GameObjects.Text);
    e.default = s
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = function(t) {
        function e(e) {
            var n = t.call(this, e, 0, 0, "", {}) || this;
            n.setColor("#000000");
            var o = e.r;
            return n.setFontSize(o),
            "center" === e.game.myConfig.statusBarAlign && (n.setX(e.game.canvas.width / 2),
            n.setOrigin(.5, 0)),
            n.setPadding(o, o, o, o),
            n
        }
        return r(e, t),
        e
    }(Phaser.GameObjects.Text);
    e.default = i
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = function(t) {
        function e(e) {
            var n = t.call(this, e, 0, 0, "", {}) || this;
            n.setColor("#000000"),
            n.setPosition(e.game.canvas.width, e.game.canvas.height),
            n.setOrigin(1, 1);
            var o = e.r;
            return n.setFontSize(.8 * o),
            n.setPadding(o, o, o, o),
            n.setText(e.game.myConfig.credit),
            n
        }
        return r(e, t),
        e
    }(Phaser.GameObjects.Text);
    e.default = i
}
, function(t, e, n) {
    "use strict";
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var o = n(0)
      , r = function() {
        function t(t, e, n, o) {
            this.i = e,
            this.j = n,
            this.isWall = o,
            this.distance = 1 / 0,
            this.parent = t,
            this.isEdge = this.i <= 0 || this.i >= this.parent.w - 1 || this.j <= 0 || this.j >= this.parent.h - 1
        }
        return Object.defineProperty(t.prototype, "routesCount", {
            get: function() {
                var t = this;
                if (void 0 === this._routesCount)
                    if (this.isEdge)
                        this._routesCount = 1;
                    else {
                        var e = 0;
                        this.neighbours.forEach(function(n) {
                            null === n || n.isWall || n.distance < t.distance && (e += n.routesCount)
                        }),
                        this._routesCount = e
                    }
                return this._routesCount
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(t.prototype, "neighbours", {
            get: function() {
                var t = this;
                if (void 0 === this._neighbours) {
                    var e = o.default.getNeighbours(this.i, this.j);
                    this._neighbours = e.map(function(e) {
                        return t.parent.getBlock(e.i, e.j)
                    })
                }
                return this._neighbours
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(t.prototype, "directions", {
            get: function() {
                var t = this
                  , e = [];
                return this.neighbours.forEach(function(n, o) {
                    null === n || n.isWall || n.distance < t.distance && e.push(o)
                }),
                e
            },
            enumerable: !0,
            configurable: !0
        }),
        Object.defineProperty(t.prototype, "direction", {
            get: function() {
                var t = this
                  , e = 0
                  , n = -1;
                return this.directions.forEach(function(o) {
                    var r = t.neighbours[o];
                    r.routesCount > e && (e = r.routesCount,
                    n = o)
                }),
                n
            },
            enumerable: !0,
            configurable: !0
        }),
        t
    }()
      , i = function() {
        function t(t) {
            var e = this;
            if (this.w = t.length,
            this.w <= 0)
                throw new Error("empty blocks");
            this.h = t[0].length,
            this.blocks = t.map(function(n, o) {
                return n.map(function(n, i) {
                    return new r(e,o,i,t[o][i])
                })
            })
        }
        return t.prototype.getBlock = function(t, e) {
            return t >= 0 && t < this.w && e >= 0 && e < this.h ? this.blocks[t][e] : null
        }
        ,
        t.prototype.calcAllDistances = function() {
            var t = [];
            this.blocks.forEach(function(e) {
                e.forEach(function(e) {
                    e.isEdge && !e.isWall && (e.distance = 0,
                    t.push(e))
                })
            });
            for (var e = function() {
                var e = t.shift();
                e.neighbours.forEach(function(n) {
                    null === n || n.isEdge || n.isWall || n.distance > e.distance + 1 && (n.distance = e.distance + 1,
                    t.indexOf(n) < 0 && t.push(n))
                })
            }; t.length > 0; )
                e()
        }
        ,
        t.prototype.toString = function() {
            for (var t = [], e = 0; e < this.h; e++) {
                for (var n = [], o = 0; o < this.w; o++) {
                    var r = this.getBlock(o, e);
                    r.isWall ? n.push("*") : r.distance === 1 / 0 ? n.push("-") : n.push(r.distance)
                }
                var i = n.join(" ");
                1 == (1 & e) && (i = " " + i),
                t.push(i)
            }
            return t.join("\n")
        }
        ,
        t.prototype.toString2 = function() {
            for (var t = [], e = 0; e < this.h; e++) {
                for (var n = [], o = 0; o < this.w; o++) {
                    var r = this.getBlock(o, e);
                    r.isWall ? n.push("*") : r.routesCount === 1 / 0 ? n.push("-") : n.push(r.routesCount)
                }
                var i = n.join(" ");
                1 == (1 & e) && (i = " " + i),
                t.push(i)
            }
            return t.join("\n")
        }
        ,
        t
    }();
    e.nearestSolver = function(t, e, n) {
        var o = new i(t);
        o.calcAllDistances();
        var r = o.getBlock(e, n).directions;
        return r.length > 0 ? r[0] : -1
    }
    ,
    e.default = function(t, e, n) {
        var o = new i(t);
        return o.calcAllDistances(),
        o.getBlock(e, n).direction
    }
}
, function(t, e, n) {
    "use strict";
    var o, r = this && this.__extends || (o = function(t, e) {
        return (o = Object.setPrototypeOf || {
            __proto__: []
        }instanceof Array && function(t, e) {
            t.__proto__ = e
        }
        || function(t, e) {
            for (var n in e)
                e.hasOwnProperty(n) && (t[n] = e[n])
        }
        )(t, e)
    }
    ,
    function(t, e) {
        function n() {
            this.constructor = t
        }
        o(t, e),
        t.prototype = null === e ? Object.create(e) : (n.prototype = e.prototype,
        new n)
    }
    );
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var i = n(28)
      , s = function(t) {
        function e(e, n, o, r) {
            var i = t.call(this, e, n, void 0, r) || this;
            return i.rawData = o,
            i
        }
        return r(e, t),
        e.prototype.load = function() {
            this.state === Phaser.Loader.FILE_POPULATED ? this.loader.nextFile(this, !0) : this.xhrLoader = i.default(this, this.loader.xhr)
        }
        ,
        e
    }(Phaser.Loader.FileTypes.SVGFile);
    e.default = s
}
, function(t, e, n) {
    "use strict";
    Object.defineProperty(e, "__esModule", {
        value: !0
    });
    var o = n(29);
    e.default = function(t, e) {
        var n = new o.default;
        return n.open("GET", t.src, e.async, e.user, e.password),
        n.responseType = t.xhrSettings.responseType,
        n.timeout = e.timeout,
        e.header && e.headerValue && n.setRequestHeader(e.header, e.headerValue),
        e.requestedWith && n.setRequestHeader("X-Requested-With", e.requestedWith),
        e.overrideMimeType && n.overrideMimeType(e.overrideMimeType),
        n.onload = t.onLoad.bind(t, n),
        n.onerror = t.onError.bind(t),
        n.onprogress = t.onProgress.bind(t),
        n.send(),
        setTimeout(function() {
            n.respond(200, {
                "Content-Type": "application/octet-stream"
            }, t.rawData)
        }, 1),
        n
    }
}
, function(t, e, n) {
    "use strict";
    n.r(e);
    /**
     * Minimal Event interface implementation
     *
     * Original implementation by Sven Fuchs: https://gist.github.com/995028
     * Modifications and tests by Christian Johansen.
     *
     * @author Sven Fuchs (svenfuchs@artweb-design.de)
     * @author Christian Johansen (christian@cjohansen.no)
     * @license BSD
     *
     * Copyright (c) 2011 Sven Fuchs, Christian Johansen
     */
    var o = function(t, e, n, o) {
        this.type = t,
        this.bubbles = e,
        this.cancelable = n,
        this.target = o
    };
    o.prototype = {
        stopPropagation: function() {},
        preventDefault: function() {
            this.defaultPrevented = !0
        }
    };
    var r = {
        100: "Continue",
        101: "Switching Protocols",
        200: "OK",
        201: "Created",
        202: "Accepted",
        203: "Non-Authoritative Information",
        204: "No Content",
        205: "Reset Content",
        206: "Partial Content",
        300: "Multiple Choice",
        301: "Moved Permanently",
        302: "Found",
        303: "See Other",
        304: "Not Modified",
        305: "Use Proxy",
        307: "Temporary Redirect",
        400: "Bad Request",
        401: "Unauthorized",
        402: "Payment Required",
        403: "Forbidden",
        404: "Not Found",
        405: "Method Not Allowed",
        406: "Not Acceptable",
        407: "Proxy Authentication Required",
        408: "Request Timeout",
        409: "Conflict",
        410: "Gone",
        411: "Length Required",
        412: "Precondition Failed",
        413: "Request Entity Too Large",
        414: "Request-URI Too Long",
        415: "Unsupported Media Type",
        416: "Requested Range Not Satisfiable",
        417: "Expectation Failed",
        422: "Unprocessable Entity",
        500: "Internal Server Error",
        501: "Not Implemented",
        502: "Bad Gateway",
        503: "Service Unavailable",
        504: "Gateway Timeout",
        505: "HTTP Version Not Supported"
    };
    var i = {
        "Accept-Charset": !0,
        "Accept-Encoding": !0,
        Connection: !0,
        "Content-Length": !0,
        Cookie: !0,
        Cookie2: !0,
        "Content-Transfer-Encoding": !0,
        Date: !0,
        Expect: !0,
        Host: !0,
        "Keep-Alive": !0,
        Referer: !0,
        TE: !0,
        Trailer: !0,
        "Transfer-Encoding": !0,
        Upgrade: !0,
        "User-Agent": !0,
        Via: !0
    };

    function s(t, e) {
        e.addEventListener(t, function(n) {
            var o = e["on" + t];
            o && "function" == typeof o && o.call(n.target, n)
        })
    }

    function a() {
        this._eventListeners = {};
        for (var t = ["loadstart", "progress", "load", "abort", "loadend"], e = t.length - 1; e >= 0; e--)
            s(t[e], this)
    }

    function l() {
        a.call(this),
        this.readyState = l.UNSENT,
        this.requestHeaders = {},
        this.requestBody = null,
        this.status = 0,
        this.statusText = "",
        this.upload = new a
    }
    a.prototype = {
        addEventListener: function(t, e) {
            this._eventListeners[t] = this._eventListeners[t] || [],
            this._eventListeners[t].push(e)
        },
        removeEventListener: function(t, e) {
            for (var n = this._eventListeners[t] || [], o = 0, r = n.length; o < r; ++o)
                if (n[o] == e)
                    return n.splice(o, 1)
        },
        dispatchEvent: function(t) {
            for (var e = t.type, n = this._eventListeners[e] || [], o = 0; o < n.length; o++)
                "function" == typeof n[o] ? n[o].call(this, t) : n[o].handleEvent(t);
            return !!t.defaultPrevented
        },
        _progress: function(t, e, n) {
            var r = new o("progress");
            r.target = this,
            r.lengthComputable = t,
            r.loaded = e,
            r.total = n,
            this.dispatchEvent(r)
        }
    },
    l.prototype = new a,
    l.UNSENT = 0,
    l.OPENED = 1,
    l.HEADERS_RECEIVED = 2,
    l.LOADING = 3,
    l.DONE = 4;
    var d = {
        UNSENT: 0,
        OPENED: 1,
        HEADERS_RECEIVED: 2,
        LOADING: 3,
        DONE: 4,
        async: !0,
        withCredentials: !1,
        open: function(t, e, n, o, r) {
            this.method = t,
            this.url = e,
            this.async = "boolean" != typeof n || n,
            this.username = o,
            this.password = r,
            this.responseText = null,
            this.responseXML = null,
            this.responseURL = e,
            this.requestHeaders = {},
            this.sendFlag = !1,
            this._readyStateChange(l.OPENED)
        },
        setRequestHeader: function(t, e) {
            if (u(this),
            i[t] || /^(Sec-|Proxy-)/.test(t))
                throw new Error('Refused to set unsafe header "' + t + '"');
            this.requestHeaders[t] ? this.requestHeaders[t] += "," + e : this.requestHeaders[t] = e
        },
        send: function(t) {
            if (u(this),
            !/^(get|head)$/i.test(this.method)) {
                var e = !1;
                Object.keys(this.requestHeaders).forEach(function(t) {
                    "content-type" === t.toLowerCase() && (e = !0)
                }),
                e || (t || "").toString().match("FormData") || (this.requestHeaders["Content-Type"] = "text/plain;charset=UTF-8"),
                this.requestBody = t
            }
            this.errorFlag = !1,
            this.sendFlag = this.async,
            this._readyStateChange(l.OPENED),
            "function" == typeof this.onSend && this.onSend(this),
            this.dispatchEvent(new o("loadstart",!1,!1,this))
        },
        abort: function() {
            this.aborted = !0,
            this.responseText = null,
            this.errorFlag = !0,
            this.requestHeaders = {},
            this.dispatchEvent(new o("abort",!1,!1,this)),
            this.readyState > l.UNSENT && this.sendFlag && (this._readyStateChange(l.UNSENT),
            this.sendFlag = !1),
            "function" == typeof this.onerror && this.onerror()
        },
        getResponseHeader: function(t) {
            if (this.readyState < l.HEADERS_RECEIVED)
                return null;
            if (/^Set-Cookie2?$/i.test(t))
                return null;
            for (var e in t = t.toLowerCase(),
            this.responseHeaders)
                if (e.toLowerCase() == t)
                    return this.responseHeaders[e];
            return null
        },
        getAllResponseHeaders: function() {
            if (this.readyState < l.HEADERS_RECEIVED)
                return "";
            var t = "";
            for (var e in this.responseHeaders)
                this.responseHeaders.hasOwnProperty(e) && !/^Set-Cookie2?$/i.test(e) && (t += e + ": " + this.responseHeaders[e] + "\r\n");
            return t
        },
        overrideMimeType: function(t) {
            "string" == typeof t && (this.forceMimeType = t.toLowerCase())
        },
        _readyStateChange: function(t) {
            this.readyState = t,
            "function" == typeof this.onreadystatechange && this.onreadystatechange(new o("readystatechange")),
            this.dispatchEvent(new o("readystatechange")),
            this.readyState == l.DONE && this.dispatchEvent(new o("load",!1,!1,this)),
            this.readyState != l.UNSENT && this.readyState != l.DONE || this.dispatchEvent(new o("loadend",!1,!1,this))
        },
        _setResponseHeaders: function(t) {
            for (var e in this.responseHeaders = {},
            t)
                t.hasOwnProperty(e) && (this.responseHeaders[e] = t[e]);
            this.forceMimeType && (this.responseHeaders["Content-Type"] = this.forceMimeType),
            this.async ? this._readyStateChange(l.HEADERS_RECEIVED) : this.readyState = l.HEADERS_RECEIVED
        },
        _setResponseBody: function(t) {
            !function(t) {
                if (t.readyState == l.DONE)
                    throw new Error("Request done")
            }(this),
            function(t) {
                if (t.async && t.readyState != l.HEADERS_RECEIVED)
                    throw new Error("No headers received")
            }(this),
            function(t) {
                if ("string" != typeof t) {
                    var e = new Error("Attempted to respond to fake XMLHttpRequest with " + t + ", which is not a string.");
                    throw e.name = "InvalidBodyException",
                    e
                }
            }(t);
            var e = this.chunkSize || 10
              , n = 0;
            this.responseText = "";
            do {
                this.async && this._readyStateChange(l.LOADING),
                this.responseText += t.substring(n, n + e),
                n += e
            } while (n < t.length);
            var o, r, i = this.getResponseHeader("Content-Type");
            if (this.responseText && (!i || /(text\/xml)|(application\/xml)|(\+xml)/.test(i)))
                try {
                    this.responseXML = (o = this.responseText,
                    "undefined" != typeof DOMParser ? r = (new DOMParser).parseFromString(o, "text/xml") : ((r = new ActiveXObject("Microsoft.XMLDOM")).async = "false",
                    r.loadXML(o)),
                    r)
                } catch (t) {}
            this.async ? this._readyStateChange(l.DONE) : this.readyState = l.DONE
        },
        respond: function(t, e, n) {
            this._setResponseHeaders(e || {}),
            this.status = "number" == typeof t ? t : 200,
            this.statusText = r[this.status],
            this._setResponseBody(n || "")
        }
    };
    for (var h in d)
        l.prototype[h] = d[h];

    function u(t) {
        if (t.readyState !== l.OPENED)
            throw new Error("INVALID_STATE_ERR");
        if (t.sendFlag)
            throw new Error("INVALID_STATE_ERR")
    }
    e.default = l
}
]);
