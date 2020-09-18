module Main exposing (main)


import Browser
import Camera3d
import Color exposing (Color)
import Html
import Length
import Pixels
import Point3d
import Scene3d
import Scene3d.Material as Material
import Scene3d.Mesh as Mesh
import Task
import Viewpoint3d
import WebGL.Texture
import Direction3d
import Array
import TriangularMesh

tileSheetSizePx : Float
tileSheetSizePx = 512.0

tileSizePx : Float
tileSizePx = 128.0

type alias UVs =
    { bottomLeft: (Float, Float)
    , bottomRight: (Float, Float)
    , topLeft: (Float, Float)
    , topRight: (Float, Float)
    }

tilePositionToUVs : (Float, Float) -> UVs
tilePositionToUVs (x, y) =
    let
        (dx, dy) = (x * tileSizePx, y * tileSizePx)
    in

    { bottomLeft = (dx  / tileSheetSizePx , dy  / tileSheetSizePx)
    , bottomRight = (dx / tileSheetSizePx , (dy + tileSizePx) / tileSheetSizePx)
    , topLeft = ((dx + tileSizePx) / tileSheetSizePx , dy / tileSheetSizePx)
    , topRight = ((dx + tileSizePx) / tileSheetSizePx, (dy + tileSizePx) / tileSheetSizePx )
    }



tileMesh : (Float, Float) -> Mesh.Unlit coordinates
tileMesh tileSheetPosition =
    let
        uvs =
            tilePositionToUVs tileSheetPosition

        vertices =
            Array.fromList
                [ { position = Point3d.meters 0 1 0, uv = uvs.bottomLeft  }
                , { position = Point3d.meters 1 1 0, uv = uvs.bottomRight  }
                , { position = Point3d.meters 0 0 0, uv = uvs.topLeft  }
                , { position = Point3d.meters 1 0 0, uv = uvs.topRight  }
                ]
        faces =
            [ ( 0, 1, 3 ), ( 0, 3, 2 ) ]
    in
        TriangularMesh.indexed vertices faces
        |> Mesh.texturedTriangles


exampleMesh = tileMesh (2, 4)

type Model
    = Loading -- Waiting for texture to load
    | Loaded (Material.Texture Color) -- Successfully loaded texture
    | Error WebGL.Texture.Error -- Error occurred when loading texture


type Msg
    = GotTexture (Result WebGL.Texture.Error (Material.Texture Color))


textureUrl : String
textureUrl =
    "/assets/road-tilesheet_annotated.png"


init : () -> ( Model, Cmd Msg )
init () =
    -- Attempt to load texture, using trilinear filtering for maximum smoothness
    ( Loading
    , Task.attempt GotTexture (Material.loadWith Material.trilinearFiltering textureUrl)
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        GotTexture (Ok texture) ->
            -- Successfully loaded the texture
            ( Loaded texture, Cmd.none )

        GotTexture (Err error) ->
            -- Network error, bad image dimensions etc.
            ( Error error, Cmd.none )


view : Model -> Browser.Document Msg
view model =
    let
        -- Construct a fixed camera
        camera =
            Camera3d.orthographic
                { viewpoint =
                    Viewpoint3d.lookAt
                        { focalPoint = Point3d.meters 0 0 0
                        , eyePoint = Point3d.meters 0 0 5
                        , upDirection = Direction3d.positiveZ
                        }
                , viewportHeight = Length.meters 10
                }
    in
    { title = "Texture"
    , body =
        case model of
            Loading ->
                -- Waiting for texture to load
                [ Html.text "Loading..." ]

            Loaded texture ->
                let
                    tileSheet = Material.texturedColor texture
                in
                -- Texture loaded successfully, render a scene using it
                [ Scene3d.unlit
                    { camera = camera
                    , dimensions = ( Pixels.int 1024, Pixels.int 1024 )
                    , background = Scene3d.backgroundColor <| Color.rgb255 22 22 22
                    , clipDepth = Length.meters 0.1
                    , entities =
                        [ Scene3d.mesh tileSheet exampleMesh
                        ]
                    }
                ]

            Error error ->
                -- Use a placeholder error message if texture loading fails
                [ Html.text (Debug.toString error) ]
    }


main : Program () Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , view = view
        , subscriptions = always Sub.none
        }
