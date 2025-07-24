// this file/ library to include any C code that is not offered by the RUST crates.
// rust can also provide a C api for most things. 
#include <stdio.h>
#include <stdlib.h>
#include "./include/libgame.h"


GLenum libgame_gl_create_shader_object(GLenum shader_kind, GLuint *shader_id) {
    // clear the error flag
    while (glGetError() !=0) {}
    // create the shader object
    *shader_id = glCreateShader(shader_kind);
    // check for errors
    GLenum status = glGetError();
    return status;
}

GLenum libgame_gl_create_shader_program_object(GLuint *program_id) {
    while(glGetError() !=0) {}
    *program_id = glCreateProgram();
    GLenum status = glGetError();
    return status;
}

GLenum libgame_gl_attach_program_to_shader(GLuint program_id, GLenum shader_id) {
    while (glGetError() !=0) {}
    glAttachShader(program_id,shader_id);
    GLenum status = glGetError();
    return status;
    
}
GLboolean libgame_gl_shader_compiler_is_supported(void) {
    GLboolean compiler_is_supported = GL_FALSE;
    glGetBooleanv(GL_SHADER_COMPILER,&compiler_is_supported);
    return compiler_is_supported;
}

void *libgame_compile_shader(const char *path, GLenum shader_kind,GLuint *out_program_id,GLuint *out_shader_id) {
    // Check for shader compiler support

    if(!libgame_gl_shader_compiler_is_supported()) {
        puts("OpenGL shader compiler is not supported on this instance");
        return;
    }

    // LOAD AND COMPILE SHADERS
    FILE *fp = NULL;
    
        errno_t ferror = fopen_s(&fp,path,"rb");
        if (fp == NULL) {
            puts("Failed to open file");
            return;
        }
    
    if (fseek(fp,0,SEEK_END)!=0) {
        puts("Failed to seek to the end of the shader file");
        fclose(fp);
        return;
    }
    long int file_size = ftell(fp);

    if(fseek(fp,0,SEEK_SET)!=0) {
        puts("failed to seek to the start of the shader file");
        fclose(fp);
        return;
    }
    
    GLchar* buffer;
    void *mem = malloc(file_size + 1);
    if(mem == NULL) {
        fclose(fp);
        return;
    }
    memset(mem,0,file_size);
    buffer = (GLchar*)mem;

    size_t bytes_read = fread(buffer,1,file_size,fp);

    int close_status = fclose(fp);
    if (close_status != 0){
        char string[255]= {0};
        sprintf_s(string,sizeof(string),"Failed to close the shader file handle: fclose returned:%d",close_status);
        puts(string);
        exit(-1);
    }
    
    if ((long)bytes_read < file_size) {
        char error[255] = {0};
        sprintf_s(error,sizeof(error),"Only part of the shader file was read: bytes_read: %ld file_size: %zd",file_size,bytes_read);
        puts(error);
        free(buffer);
        return;
    }
    // create the shader object
    GLuint shader_id = 0;
    
    GLenum status = libgame_gl_create_shader_object(shader_kind,&shader_id);
    if (status !=0 || shader_id <=0) {
        char error[255] = {0};
        sprintf_s(error,sizeof(error),"Failed to create the shader object: status %d. shader_id %d",status,shader_id);
        puts(error);
        free(buffer);
        return;
    }

    GLuint program_id = 0;
    if (libgame_gl_create_shader_program_object(&program_id) != GL_NO_ERROR || program_id <=0) {
        puts("Failed to create the program object");
        free(buffer);
        return;
    }
    if(libgame_gl_attach_program_to_shader(program_id,shader_id) != GL_NO_ERROR) {
        fclose(fp);
        free(buffer);
        puts("Failed to attach shader object to program object");
    }    
    // set the shader source
    glShaderSource(shader_id,1,&buffer,NULL);
    free(buffer);
    // if this was sccessfull
    // compile the shader
    glCompileShader(shader_id);
    GLint success = GL_FALSE;
    glGetShaderiv(shader_id,GL_COMPILE_STATUS,&success);
    if(success!=GL_TRUE){
        puts("Failed to compile shader program");
    }
    

    // link the program
    glLinkProgram(program_id);
    GLint is_linked = 0;
    glGetProgramiv(program_id,GL_LINK_STATUS,&is_linked);
    if(!is_linked){
        GLint logSize = 0;
        glGetProgramiv(program_id,GL_INFO_LOG_LENGTH,&logSize);
        printf("logsize %d",logSize);

        // allocate some memory on the heap for the log
        puts("allocating memory for buffer");
        void *mem = malloc((logSize * sizeof(GLchar) + 1));
        if (mem == NULL) {
            puts("buffer allocation failed");
            exit(-1);
        }
        memset(mem,0,logSize * sizeof(GLchar));
        GLchar* infologbuffer = (GLchar *)mem;
        infologbuffer[logSize * sizeof(GLchar) + 1] = 0;
        GLsizei actual_len = 0;
        glGetProgramInfoLog(program_id,logSize * sizeof(GLchar),&actual_len,&infologbuffer);
        FILE *infolog = fopen("shaderlog.txt","wb");
        if (infolog == NULL) {
            puts("failed to open log file");
            return;
        }
        printf("len %d",actual_len);
        fwrite(&infologbuffer,sizeof(GLchar),actual_len,infolog);
        fflush(infolog);
        if(fclose(infolog) !=0) {
            puts("failed to close log file");
        }
        free(infologbuffer);
    }
    *out_program_id = program_id;
    *out_shader_id = shader_id;
}


int main() {
    // intialize glfw
    glfwInitHint(GLFW_OPENGL_DEBUG_CONTEXT,GL_TRUE);
    glfwInitHint(GLFW_CONTEXT_VERSION_MAJOR,3);
    glfwInitHint(GLFW_CONTEXT_VERSION_MINOR,0);
    glfwInitHint(GLFW_OPENGL_FORWARD_COMPAT,0);
    glfwInit();

    // initialize window
    GLFWwindow *window = glfwCreateWindow(800,800,"A game window",NULL,NULL);
    if(window == NULL) {
        glfwTerminate();
        return -1;
    }
    // set the context
    glfwMakeContextCurrent(window);

    // init glad loader
    gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);

    // initialize opengl
    glClearColor(0.5,0.5,0.5,1.0);

    // vertex shader
    GLuint vertex_program_id,vertex_shader_id = 0;
    libgame_compile_shader("vertex_shader.glsl",GL_VERTEX_SHADER,&vertex_program_id,&vertex_shader_id);
    glUseProgram(vertex_program_id);

    // fragment shader
    GLuint fragment_program_id,fragment_shader_id = 0;
    libgame_compile_shader("fragment_shader.glsl",GL_VERTEX_SHADER,&fragment_program_id,&fragment_shader_id);
    glUseProgram(fragment_program_id);


    // create a vertex buffer object
    GLuint VBO = 0;
    glGenBuffers(1,&VBO);
    glBindBuffer(GL_ARRAY_BUFFER,VBO);

    // we are rendering a single triangle to the screen
        float vertices[] = {
        0.0f, 0.0f, 0.0f, // Point 1
        0.5f, 0.5f, 0.0f, // Point 2
        -0.5f, 0.0f, 0.0f  // Point 3
    };
    
    
    while (!glfwWindowShouldClose(window)) {
        // poll for events
        glfwPollEvents();
        
        
        
        // the viewport
        int width = 1;
        int height = 1;
        glfwGetWindowSize(window,&width,&height);
        glViewport(0,0,width,height);
        // clear the framebuffer
        glClear(GL_COLOR_BUFFER_BIT);
        
        // draw
        glBufferData(GL_ARRAY_BUFFER,sizeof(vertices),vertices,GL_STATIC_DRAW);
        glDrawArrays(GL_POINT,0,sizeof(vertices));

        // swap buffers
        glfwSwapBuffers(window);

    

    }



    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}