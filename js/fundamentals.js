/* Licensed under a BSD license. See license.html for license */
"use strict";

// var vertexShaderSource = `#version 300 es
// uniform mat4 u_worldViewProjection;
// uniform vec3 u_lightWorldPos;
// uniform mat4 u_world;
// uniform mat4 u_viewInverse;
// uniform mat4 u_worldInverseTranspose;
//
// in vec4 a_position;
// in vec3 a_normal;
//
// out vec4 v_position;
// out vec3 v_normal;
// out vec3 v_surfaceToLight;
// out vec3 v_surfaceToView;
//
// void main() {
//   v_position = (u_worldViewProjection * a_position);
//   v_normal = (u_worldInverseTranspose * vec4(a_normal, 0)).xyz;
//   v_surfaceToLight = u_lightWorldPos - (u_world * a_position).xyz;
//   v_surfaceToView = (u_viewInverse[3] - (u_world * a_position)).xyz;
//   gl_Position = v_position;
// }
// `;
//
// var fragmentShaderSource = `#version 300 es
// precision mediump float;
//
// in vec4 v_position;
// in vec3 v_normal;
// in vec3 v_surfaceToLight;
// in vec3 v_surfaceToView;
//
// out vec4 outColor;
//
// uniform vec4 u_lightColor;
// uniform vec4 u_ambient;
// uniform vec4 u_diffuse;
// uniform vec4 u_specular;
// uniform float u_shininess;
// uniform float u_specularFactor;
//
// vec4 lit(float l ,float h, float m) {
//   return vec4(1.0,
//               pow(l, 2.5),
//               (l > 0.0) ? pow(max(0.0, h), m) : 0.0,
//               1.0);
// }
//
// void main() {
//   float depth = gl_FragCoord.z / gl_FragCoord.w;
//   vec4 diffuseColor = u_diffuse;
//   vec3 a_normal = normalize(v_normal);
//   vec3 surfaceToLight = normalize(v_surfaceToLight);
//   vec3 surfaceToView = normalize(v_surfaceToView);
//   vec3 halfVector = normalize(surfaceToLight + surfaceToView);
//   vec4 litR = lit(dot(a_normal, surfaceToLight),
//                     dot(a_normal, halfVector), u_shininess);
//   outColor = vec4((
//   u_lightColor * (diffuseColor * litR.y + diffuseColor * u_ambient +
//                 u_specular * litR.z * u_specularFactor)).rgb,
//       diffuseColor.a);
// }
// `;

/** https://webgl2fundamentals.org/ */
function getModel(){
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (xhr.readyState === XMLHttpRequest.DONE) {
            let response = xhr.responseText;
            let m = JSON.parse(response);
            main(m)
        }
    };
    xhr.open('GET', 'js/model.json', true);
    xhr.send(null);
}

function getShaders(callback) {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
        if (xhr.readyState === XMLHttpRequest.DONE) {
            let response = xhr.responseText;
            callback(response)
        }
    };
    xhr.open('GET', 'js/fundamentals.glsl', true);
    xhr.send(null);
}

function main(model) {
    /** Get A WebGL context */

    let canvas = document.getElementById("glCanvas");
    let gl = canvas.getContext("webgl2", { alpha: false }); /** This is bad, using sync like this! */

    if (!gl) {return;}

    /** setup GLSL program */
    getShaders(function (sources) {
        let fragIndex = sources.search("//FRAGMENT");
        let vertexShaderSource = sources.substr(0, fragIndex).replace("//VERTEX", "");
        let fragmentShaderSource = sources.substr(fragIndex, sources.length-1).replace("//FRAGMENT", "");

        let programInfo = twgl.createProgramInfo(gl, [vertexShaderSource, fragmentShaderSource]);
        twgl.setAttributePrefix("a_");  /** Tell the webglUtils to match position with a_position etc.. */
        let bufferInfo = twgl.createBufferInfoFromArrays(gl, model);
        let vao = gl.createVertexArray();
        gl.bindVertexArray(vao);
        twgl.setBuffersAndAttributes(gl, programInfo, bufferInfo);

        function degToRad(d) {
            return d * Math.PI / 180;
        }

        let fieldOfViewRadians = degToRad(40);
        let zNear = 1;
        let zFar  = 500;

        let uniformsThatAreTheSameForAllObjects = {
            u_lightWorldPos:         [5000, 3000, 10000],
            u_viewInverse:           m4.identity(),
            u_lightColor:            [1, 1, 1, 1],
        };

        let uniformsThatAreComputedForEachObject = {
            u_worldViewProjection:   m4.identity(),
            u_world:                 m4.identity(),
            u_worldInverseTranspose: m4.identity(),
        };

        let materialUniforms = {
            u_ambient:               [.75, 0, 0, 0],
            u_diffuse:               [1, 1, 1, 1],
            u_specular:              [1, 1, 1, 1],
            u_shininess:             100,
            u_specularFactor:        1,
        };

        let randomSeed = 0;
        let RANDOM_RANGE = Math.pow(2, 32);

        function pseudoRandom() {
            return (randomSeed =
                (134775813 * randomSeed + 1) %
                RANDOM_RANGE) / RANDOM_RANGE;
        }

        function r(min, max) {
            if (max === undefined) {
                max = min;
                min = 0;
            }
            return pseudoRandom() * (max - min) + min;
        }

        function resetPseudoRandom() {
            randomSeed = 0;
        }

        requestAnimationFrame(drawScene);

        /** Draw the scene. */
        function drawScene(time) {
            twgl.resizeCanvasToDisplaySize(canvas);

            time *= 0.001;  /** convert to seconds */

            /** Set the viewport to match the canvas */
            gl.viewport(0, 0, canvas.width, canvas.height);

            /** Clear the canvas AND the depth buffer. */
            gl.clearColor(71/255 * .8, 255/255 * .8, 176/255 * .8, 1);
            gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

            gl.enable(gl.CULL_FACE);
            gl.enable(gl.DEPTH_TEST);

            /** Compute the projection matrix */
            let aspect = canvas.clientWidth / canvas.clientHeight;
            let projectionMatrix = m4.perspective(fieldOfViewRadians, aspect, zNear, zFar);

            /** Compute the camera's matrix using look at. */
            let orbitRadius = 100;
            let orbitTime = time * -0.05;
            let cameraPosition = [Math.cos(orbitTime) * orbitRadius, Math.sin(orbitTime * 1.123) * orbitRadius, Math.sin(orbitTime) * orbitRadius];
            let target = [0, 0, 0];
            let up = [0, 1, 0];
            let cameraMatrix = m4.lookAt(cameraPosition, target, up, uniformsThatAreTheSameForAllObjects.u_viewInverse);

            /** Make a view matrix from the camera matrix. */
            let viewMatrix = m4.inverse(cameraMatrix);

            gl.useProgram(programInfo.program);

            resetPseudoRandom();

            /** Setup all the needed attributes. */
            gl.bindVertexArray(vao);

            /** Set the uniforms that are the same for all objects. */
            twgl.setUniforms(programInfo, uniformsThatAreTheSameForAllObjects);

            /** Draw objects */
            let num = 4;
            let spread = 20;
            for (let zz = -num; zz <= num; ++zz) {
                for (let yy = -num; yy <= num; ++yy) {
                    for (let xx = -num; xx <= num; ++xx) {
                        let matrix = m4.identity(uniformsThatAreComputedForEachObject.u_world);
                        matrix = m4.translate(matrix, xx * spread, yy * spread, zz * spread);
                        matrix = m4.axisRotate(matrix, m4.normalize([r(-1, 1), r(-1, 1), r(-1, 1)]), time * .2 + r(Math.PI * 2));
                        matrix = m4.scale(matrix, 20, 20, 20, uniformsThatAreComputedForEachObject.u_world);
                        let worldMatrix = matrix;

                        /** Multiply the matrices. */
                        matrix = m4.multiply(viewMatrix, worldMatrix);
                        m4.multiply(projectionMatrix, matrix, uniformsThatAreComputedForEachObject.u_worldViewProjection);
                        m4.transpose(m4.inverse(worldMatrix), uniformsThatAreComputedForEachObject.u_worldInverseTranspose);

                        /** Set the uniforms we just computed */
                        twgl.setUniforms(programInfo, uniformsThatAreComputedForEachObject);

                        /** Set the uniforms that are specific to the this object. */
                        twgl.setUniforms(programInfo, materialUniforms);

                        /** Draw the geometry. */
                        gl.drawArrays(gl.TRIANGLES, 0, bufferInfo.numElements);
                    }
                }
            }
            requestAnimationFrame(drawScene);
        }
    });
}

/** Set a color for this object. */
//materialUniforms.u_diffuse[0] = 1;
//materialUniforms.u_diffuse[1] = 1;
//materialUniforms.u_diffuse[2] = 1;




