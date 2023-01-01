import dearpygui.dearpygui as dpg
import numpy as np

# TODO: Implement in C++ for performance


def CalcMandelbrot(x, y, iterations, escape_constant):
    constant = np.complex128(x + (y*1j))
    nextZ = constant  # Skipping first iteration

    for i in range(iterations):
        nextZ = (nextZ*nextZ) + constant
        if abs(nextZ - constant) > escape_constant:
            return i

    return iterations


def RenderTexture(imgSize=256, iterations=3, offset=[0, 0], step=1, escape_constant=50):
    texture_data = []
    c = 1

    for i in range(0, imgSize * imgSize):
        x = (i % imgSize - (imgSize/2))*step - offset[0]
        y = (np.floor_divide(i, imgSize) - (imgSize/2))*step - offset[1]

        escape_time = CalcMandelbrot(x, y, iterations, escape_constant)
        if escape_time > iterations/2:
            factor = ((iterations - 1)-(escape_time))/(iterations - 1)

            red, green, blue = factor * \
                (200/255), factor * (25/255), factor * (25/255)
        else:
            factor = ((iterations - 1)-(escape_time))/(iterations - 1)
            factor = (factor - 0.5)*2

            red, green, blue = factor * \
                (200/255), factor * (25/255), factor * (25/255)

            red, green, blue = red + (factor*(-200/255)), green + (
                factor*(+75/255)), + (factor*(+230/255))

        texture_data.append(red)
        texture_data.append(green)
        texture_data.append(blue)
        texture_data.append(255 / 255)
        if (i+1 == imgSize*c):
            print(
                f"Progress: {'%.2f' % (((i)/((imgSize*imgSize))) * 100)}%")
            c += 1

    return texture_data


def SaveBtn_callback(sender, app_data, user_data):
    size = dpg.get_value(user_data[0])
    iterations = dpg.get_value(user_data[1])
    x = dpg.get_value(user_data[2])
    y = dpg.get_value(user_data[3])
    step = 1/dpg.get_value(user_data[4])
    escape = dpg.get_value(user_data[5])

    print("Rendering...")

    new_texture = RenderTexture(
        imgSize=size, iterations=iterations, offset=[x, y], step=step, escape_constant=escape)

    print("Image Rendered")

    dpg.set_value("texture_tag", new_texture)


def Init(imgSize):
    dpg.create_context()

    print("Initializing...")

    print("Rendering...")
    texture_data = RenderTexture(imgSize)
    print("Image Rendered")

    with dpg.texture_registry(show=False):
        dpg.add_dynamic_texture(width=imgSize, height=imgSize,
                                default_value=texture_data, tag="texture_tag")
    dpg.create_viewport(title='Custom Title', width=1300, height=1000)

    with dpg.window(label="FracRendering"):
        with dpg.group(label="Options"):
            dpg.add_text("Fractal Rendering")
            size_slider = dpg.add_input_int(
                label="Size", default_value=1024)
            x_slider = dpg.add_input_float(
                label="X Offset", default_value=0)
            y_slider = dpg.add_input_float(
                label="Y Offset", default_value=0)
            step_slider = dpg.add_input_int(
                label="Pixel Step", default_value=1)
            iterations_slider = dpg.add_input_int(
                label="Iterations", default_value=3)
            escape_slider = dpg.add_input_float(
                label="Escape Value", default_value=50)
            saveBtn = dpg.add_button(label="Save Values")

        with dpg.group():
            dpg.add_image("texture_tag")

        dpg.set_item_callback(saveBtn, SaveBtn_callback)
        dpg.set_item_user_data(
            saveBtn, [size_slider, iterations_slider, x_slider, y_slider, step_slider, escape_slider])

    dpg.setup_dearpygui()
    dpg.show_viewport()
    dpg.start_dearpygui()
    dpg.destroy_context()

    print("Initialized")


Init(1024)
