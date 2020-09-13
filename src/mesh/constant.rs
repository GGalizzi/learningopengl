#[rustfmt::skip]
pub const CUBE_VERTICES: [f32; 36 * 3] = [
    // back face
    0.0, 0.0, 0.0, // bottom-left
    0.1, 0.0, 0.0, // bottom-right    
    0.1,  0.1, 0.0, // top-right              
    0.1,  0.1, 0.0, // top-right
    0.0,  0.1, 0.0, // top-left
    0.0, 0.0, 0.0, // bottom-left                
    // front face
    0.0, 0.0,  0.1, // bottom-left
    0.1,  0.1,  0.1,  // top-right
    0.1, 0.0,  0.1,  // bottom-right        
    0.1,  0.1,  0.1,  // top-right
    0.0, 0.0,  0.1,  // bottom-left
    0.0,  0.1,  0.1,  // top-left        
    // left face
    0.0,  0.1,  0.1, // top-right
    0.0, 0.0, 0.0, // bottom-left
    0.0,  0.1, 0.0, // top-left       
    0.0, 0.0, 0.0, // bottom-left
    0.0,  0.1,  0.1, // top-right
    0.0, 0.0,  0.1, // bottom-right
    // right face
    0.1,  0.1,  0.1,  // top-left
    0.1,  0.1, 0.0,  // top-right      
    0.1, 0.0, 0.0,  // bottom-right          
    0.1, 0.0, 0.0,  // bottom-right
    0.1, 0.0,  0.1,  // bottom-left
    0.1,  0.1,  0.1,  // top-left
    // bottom face          
    0.0, 0.0, 0.0, // top-right
    0.1, 0.0,  0.1, // bottom-left
    0.1, 0.0, 0.0, // top-left        
    0.1, 0.0,  0.1, // bottom-left
    0.0, 0.0, 0.0, // top-right
    0.0, 0.0,  0.1, // bottom-right
    // top face
    0.0,  0.1, 0.0, // top-left
    0.1,  0.1, 0.0, // top-right
    0.1,  0.1,  0.1, // bottom-right                 
    0.1,  0.1,  0.1, // bottom-right
    0.0,  0.1,  0.1, // bottom-left  
    0.0,  0.1, 0.0, // top-left
];

#[rustfmt::skip]
pub const CUBE_TEXTURE_MAPPING: [f32; 36 * 2] = [
    // FRONT
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    1.0, 1.0,
    0.0, 1.0,
    0.0, 0.0,
    // Back
    0.0, 0.0, // Bottom left
    1.0, 0.0, // Bottom right
    1.0, 1.0, // Top right
    1.0, 1.0, // Top right
    0.0, 1.0, // Top left
    0.0, 0.0, // Bottom left
    // Left
    1.0, 1.0,
    0.0, 1.0,
    0.0, 0.0,
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    // Right
    1.0, 1.0,
    0.0, 1.0,
    0.0, 0.0,
    0.0, 0.0,
    1.0, 0.0,
    1.0, 1.0,
    // Bottom
    0.0, 1.0,
    1.0, 1.0,
    1.0, 0.0,
    1.0, 0.0,
    0.0, 0.0,
    0.0, 1.0,
    // Top
    0.0, 1.0,
    1.0, 1.0,
    1.0, 0.0,
    1.0, 0.0,
    0.0, 0.0,
    0.0, 1.0,
];
