import * as THREE from 'tree';

function createGlobe() {
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
  const renderer = new THREE.WebGLRenderer({ alpha: true });
  
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.getElementById('globe').appendChild(renderer.domElement);

  const geometry = new THREE.SphereGeometry(5, 32, 32);
  const material = new THREE.MeshBasicMaterial({
    color: 0x26217f,
    wireframe: true,
    transparent: true,
    opacity: 0.3
  });
  const globe = new THREE.Mesh(geometry, material);
  scene.add(globe);

  camera.position.z = 10;

  function addStar() {
    const geometry = new THREE.SphereGeometry(0.05, 24, 24);
    const material = new THREE.MeshBasicMaterial({ color: 0xffffff });
    const star = new THREE.Mesh(geometry, material);

    const [x, y, z] = Array(3).fill().map(() => THREE.MathUtils.randFloatSpread(100));
    star.position.set(x, y, z);
    scene.add(star);
  }

  Array(200).fill().forEach(addStar);

  function animate() {
    requestAnimationFrame(animate);
    globe.rotation.x += 0.001;
    globe.rotation.y += 0.002;
    renderer.render(scene, camera);
  }

  animate();

  window.addEventListener('resize', () => {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
  });
}

document.addEventListener('DOMContentLoaded', createGlobe);